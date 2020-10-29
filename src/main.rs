#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all)]
#![warn(clippy::nursery)]

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use clap::{AppSettings, Clap};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};

use crate::matrix::{Rain, RainState};

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

fn main() -> Result<()> {
    let opt = Opt::parse();

    #[cfg(feature = "twitch")]
    let namelist = twitch::get_viewers(&opt.twitch_username)?;
    #[cfg(not(feature = "twitch"))]
    let namelist = vec!["test".to_owned()];

    let mut terminal = create_terminal()?;
    let stop = create_shutdown();

    let sleep_time = Duration::from_millis(1000 / opt.fps);
    let mut state = RainState::new(Duration::from_millis(1000 / opt.dps));

    loop {
        terminal.draw(|f| {
            let size = f.size();
            f.render_stateful_widget(Rain::new(47, &namelist), size, &mut state);
        })?;

        thread::sleep(sleep_time);

        if stop.load(Ordering::Relaxed) {
            break;
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

fn create_shutdown() -> Arc<AtomicBool> {
    let stop = Arc::new(AtomicBool::new(false));
    let s = stop.clone();

    thread::spawn(move || {
        let mut keys = io::stdin().keys();

        while let Some(Ok(k)) = keys.next() {
            match k {
                Key::Esc | Key::Char('q') | Key::Ctrl('c') => break,
                _ => {}
            }
        }

        s.store(true, Ordering::Relaxed);
    });

    stop
}
