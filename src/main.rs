#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all)]
#![warn(clippy::nursery)]

use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueHint};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use matrix::{KanaBackground, KanaBackgroundState};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Margin, Rect},
    style::{Color, Style},
    widgets::{Clear, Paragraph},
    Terminal,
};

use crate::matrix::{
    Countdown, KanaBorder, KanaBorderState, KanaList, KanaListState, Rain, RainState,
};

mod matrix;
#[cfg(feature = "twitch")]
mod twitch;

#[derive(Parser)]
#[clap(about, author, version)]
struct Opt {
    /// Frames per second.
    #[clap(short, long, value_parser, default_value_t = 5)]
    fps: u64,
    /// Drops per second.
    #[clap(short, long, value_parser, default_value_t = 3)]
    dps: u64,
    #[clap(subcommand)]
    source: Option<Source>,
}

#[derive(Subcommand)]
enum Source {
    File {
        /// Location to the file containing names.
        #[clap(value_parser, value_hint=ValueHint::FilePath)]
        path: PathBuf,
    },
    #[cfg(feature = "twitch")]
    Twitch {
        /// Streamer name to load viewer names from.
        #[clap(value_parser)]
        username: String,
    },
}

#[derive(Eq, PartialEq)]
enum Showing {
    Nothing,
    Menu,
    Help,
    Time,
}

const SLEEP_TIME: Duration = Duration::from_millis(1000 / 25);

const HELP_TEXT: &str = "\
Welcome to rsmatrix a Matrix rain screensaver written in Rust.

The following commands can be used:

  - h toggle this help message
  - m toggle the menu to navigate to different areas
    - ▲ navigate menu up
    - ▼ navigate menu down
  - q quit the application\
";

fn main() -> Result<()> {
    let opt = Opt::parse();

    let namelist = if let Some(source) = opt.source {
        match source {
            Source::File { path } => load_file(path)?,
            #[cfg(feature = "twitch")]
            Source::Twitch { username } => twitch::get_viewers(&username)?,
        }
    } else {
        vec!["test".to_owned()]
    };

    let mut terminal = create_terminal()?;
    let events = create_event_listener();

    let update_speed = Duration::from_millis(1000 / opt.fps);
    let drop_speed = Duration::from_millis(1000 / opt.dps);
    let mut background_state = KanaBackgroundState::default();
    let mut state = RainState::new();
    let mut border_state = KanaBorderState::default();
    let mut list_state = KanaListState::default();
    let mut showing = Showing::Nothing;
    let mut timer_start = Instant::now();

    let list_items = &["Countdown"];

    'drawloop: loop {
        terminal.draw(|f| {
            let size = f.size();

            f.render_stateful_widget(
                KanaBackground::new(Duration::from_millis(300)),
                size,
                &mut background_state,
            );
            f.render_stateful_widget(
                Rain::new(&namelist, update_speed, drop_speed),
                size,
                &mut state,
            );

            match showing {
                Showing::Menu => {
                    let border = KanaBorder::default().title("MENU");
                    let list = KanaList::new(list_items);

                    let r = Rect::new(0, 0, 40, 4 + list_items.len() as u16);
                    let r = r.center_in(size);

                    f.render_widget(Clear, r);
                    f.render_stateful_widget(border, r, &mut border_state);

                    let r = r.inner(&Margin {
                        vertical: 2,
                        horizontal: 3,
                    });
                    f.render_stateful_widget(list, r, &mut list_state);
                }
                Showing::Help => {
                    let border = KanaBorder::default().title("HELP");
                    let help =
                        Paragraph::new(HELP_TEXT).style(Style::default().fg(Color::Indexed(47)));

                    let r = Rect::new(0, 0, 68, 13).center_in(size);

                    f.render_widget(Clear, r);
                    f.render_stateful_widget(border, r, &mut border_state);

                    let r = r.inner(&Margin {
                        vertical: 2,
                        horizontal: 3,
                    });

                    f.render_widget(help, r);
                }
                Showing::Time => {
                    let duration = Duration::from_millis(300_500)
                        .checked_sub(timer_start.elapsed())
                        .unwrap_or_default();

                    f.render_widget(Countdown { duration }, size);
                }
                Showing::Nothing => {}
            }
        })?;

        thread::sleep(SLEEP_TIME);

        while let Ok(event) = events.try_recv() {
            match event {
                KeyEvent::Quit => break 'drawloop,
                KeyEvent::ToggleMenu => {
                    showing = match showing {
                        Showing::Menu => Showing::Nothing,
                        _ => Showing::Menu,
                    }
                }
                KeyEvent::ToggleHelp => {
                    showing = match showing {
                        Showing::Help => Showing::Nothing,
                        _ => Showing::Help,
                    }
                }
                KeyEvent::MoveUp => {
                    if showing == Showing::Menu {
                        list_state.prev(list_items);
                    }
                }
                KeyEvent::MoveDown => {
                    if showing == Showing::Menu {
                        list_state.next(list_items);
                    }
                }
                KeyEvent::Select => {
                    if showing == Showing::Menu {
                        showing = match list_state.selected() {
                            0 => {
                                timer_start = std::time::Instant::now();
                                Showing::Time
                            }
                            _ => Showing::Nothing,
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn create_terminal() -> Result<Terminal<impl Backend>> {
    let stdout = RawMode::from(io::stdout())?;
    let stdout = AlternateScreen::from(stdout)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(Into::into)
}

enum KeyEvent {
    Quit,
    ToggleMenu,
    ToggleHelp,
    MoveUp,
    MoveDown,
    Select,
}

fn create_event_listener() -> Receiver<KeyEvent> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        while let Ok(event) = event::read() {
            let k = match event {
                Event::Key(k) => k,
                _ => continue,
            };

            let event = match k.code {
                KeyCode::Esc | KeyCode::Char('q') => Some(KeyEvent::Quit),
                KeyCode::Char('c') if k.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(KeyEvent::Quit)
                }
                KeyCode::Char('m') => Some(KeyEvent::ToggleMenu),
                KeyCode::Char('h') => Some(KeyEvent::ToggleHelp),
                KeyCode::Up => Some(KeyEvent::MoveUp),
                KeyCode::Down => Some(KeyEvent::MoveDown),
                KeyCode::Enter => Some(KeyEvent::Select),
                _ => None,
            };

            if let Some(event) = event {
                tx.send(event).ok();
            }
        }
    });

    rx
}

trait RectExt {
    fn center_in(self, outer: Self) -> Self;
    fn contains(self, pos: (u16, u16)) -> bool;
}

impl RectExt for Rect {
    fn center_in(self, outer: Self) -> Self {
        Self::new(
            (outer.width - self.width) / 2,
            (outer.height - self.height) / 2,
            self.width,
            self.height,
        )
    }

    fn contains(self, pos: (u16, u16)) -> bool {
        self.left() <= pos.0 && pos.0 < self.right() && self.top() <= pos.1 && pos.1 < self.bottom()
    }
}

fn load_file(path: PathBuf) -> Result<Vec<String>> {
    BufReader::new(File::open(path)?)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

struct RawMode<T>(T);

impl<T> RawMode<T> {
    fn from(value: T) -> Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(Self(value))
    }
}

impl<T: Write> Write for RawMode<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<T> Drop for RawMode<T> {
    fn drop(&mut self) {
        terminal::disable_raw_mode().ok();
    }
}

struct AlternateScreen<T: Write>(T);

impl<T: Write> AlternateScreen<T> {
    fn from(mut value: T) -> Result<Self> {
        value.execute(EnterAlternateScreen)?;
        Ok(Self(value))
    }
}

impl<T: Write> Write for AlternateScreen<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<T: Write> Drop for AlternateScreen<T> {
    fn drop(&mut self) {
        self.0.execute(LeaveAlternateScreen).ok();
    }
}
