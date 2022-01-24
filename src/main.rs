#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all)]
#![warn(clippy::nursery)]

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::{Parser, Subcommand};
use matrix::{KanaBackground, KanaBackgroundState};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Margin, Rect},
    style::Color,
    style::Style,
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
    #[clap(short, long, default_value = "5")]
    fps: u64,
    /// Drops per second.
    #[clap(short, long, default_value = "3")]
    dps: u64,
    #[clap(subcommand)]
    source: Option<Source>,
}

#[derive(Subcommand)]
enum Source {
    File {
        /// Location to the file containing names.
        path: PathBuf,
    },
    #[cfg(feature = "twitch")]
    Twitch {
        /// Streamer name to load viewer names from.
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
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
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
        let mut keys = io::stdin().keys();

        while let Some(Ok(k)) = keys.next() {
            let event = match k {
                Key::Esc | Key::Char('q') | Key::Ctrl('c') => Some(KeyEvent::Quit),
                Key::Char('m') => Some(KeyEvent::ToggleMenu),
                Key::Char('h') => Some(KeyEvent::ToggleHelp),
                Key::Up => Some(KeyEvent::MoveUp),
                Key::Down => Some(KeyEvent::MoveDown),
                Key::Char('\n') => Some(KeyEvent::Select),
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
