#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all)]
#![warn(clippy::nursery)]

use std::io;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use clap::{AppSettings, Clap};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Margin, Rect},
    style::Color,
    style::Style,
    widgets::{Clear, Paragraph},
    Terminal,
};

use crate::matrix::{KanaBorder, KanaBorderState, KanaList, KanaListState, Rain, RainState};

mod matrix;
#[cfg(feature = "twitch")]
mod twitch;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opt {
    /// Frames per second.
    #[clap(short, long, default_value = "25")]
    fps: u64,
    /// Drops per second.
    #[clap(short, long, default_value = "5")]
    dps: u64,
    #[cfg(feature = "twitch")]
    twitch_username: String,
}

#[derive(Eq, PartialEq)]
enum Showing {
    Nothing,
    Menu,
    Help,
}

const HELP_TEXT: &str = "\
Welcome to rsmatrix a Matrix rain screensaver written in Rust.

The following commands can be used:

  - h toggle this help message
  - m toggle the menu to navigate to different areas
  - q quit the application\
";

fn main() -> Result<()> {
    let opt = Opt::parse();

    #[cfg(feature = "twitch")]
    let namelist = twitch::get_viewers(&opt.twitch_username)?;
    #[cfg(not(feature = "twitch"))]
    let namelist = vec!["test".to_owned()];

    let mut terminal = create_terminal()?;
    let events = create_event_listener();

    let sleep_time = Duration::from_millis(1000 / opt.fps);
    let mut state = RainState::new(Duration::from_millis(1000 / opt.dps));
    let mut border_state = KanaBorderState::default();
    let mut list_state = KanaListState::default();
    let mut showing = Showing::Nothing;

    let list_items = &["Item 1", "Item 2", "Item 3", "Item 4"];

    'drawloop: loop {
        terminal.draw(|f| {
            let size = f.size();
            f.render_stateful_widget(Rain::new(47, &namelist), size, &mut state);

            match showing {
                Showing::Menu => {
                    let list = KanaList::new(list_items);

                    let r = Rect::new(0, 0, 40, 4 + list_items.len() as u16);
                    let r = r.center_in(size);

                    f.render_widget(Clear, r);
                    f.render_stateful_widget(KanaBorder, r, &mut border_state);

                    let r = r.inner(&Margin {
                        vertical: 2,
                        horizontal: 3,
                    });
                    f.render_stateful_widget(list, r, &mut list_state);
                }
                Showing::Help => {
                    let r = Rect::new(0, 0, 68, 11).center_in(size);

                    f.render_widget(Clear, r);
                    f.render_stateful_widget(KanaBorder, r, &mut border_state);

                    let r = r.inner(&Margin {
                        vertical: 2,
                        horizontal: 3,
                    });

                    f.render_widget(
                        Paragraph::new(HELP_TEXT).style(Style::default().fg(Color::Indexed(47))),
                        r,
                    );
                }
                Showing::Nothing => {}
            }
        })?;

        thread::sleep(sleep_time);

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
                    if matches!(showing, Showing::Menu) {
                        showing = Showing::Nothing;
                    }
                    // TODO: Do something with the menu item
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
}
