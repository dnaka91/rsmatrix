use std::time::{Duration, Instant};

use rand::distributions::Uniform;
use rand::prelude::*;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    style::{Color, Style},
    widgets::StatefulWidget,
};

#[derive(Copy, Clone)]
pub struct Rain<'a> {
    color: u8,
    namelist: &'a [String],
}

impl<'a> Rain<'a> {
    pub const fn new(color: u8, namelist: &'a [String]) -> Self {
        Self { color, namelist }
    }
}

pub struct RainState<'a> {
    elements: Vec<Element<'a>>,
    drop_speed: Duration,
    last_drop: Instant,
}

impl<'a> RainState<'a> {
    pub fn new(drop_speed: Duration) -> Self {
        Self {
            elements: Vec::new(),
            drop_speed,
            last_drop: Instant::now(),
        }
    }
}

#[derive(Default)]
struct Element<'a> {
    name: &'a str,
    trail: Vec<char>,
    pos: (u16, u16),
    active: bool,
}

impl<'a> Element<'a> {
    fn init(&mut self, rng: &mut impl Rng, area: Rect, namelist: &'a [String]) {
        self.name = namelist.choose(rng).unwrap();
        self.trail = (0..rng.sample(Uniform::new_inclusive(self.name.len(), self.name.len() * 2)))
            .map(|_| random_char(rng))
            .collect();
        self.pos = (rng.gen::<u16>() % area.right(), 0);
    }
}

fn random_char(rng: &mut impl Rng) -> char {
    match rng.next_u32() % 5 {
        0 => ('A'..='Z'),
        1 => ('0'..='9'),
        _ => ('\u{ff66}'..='\u{ff9d}'), // Half-Width Katakana
    }
    .choose(rng)
    .unwrap()
}

impl<'a> StatefulWidget for Rain<'a> {
    type State = RainState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let rng = &mut rand::thread_rng();

        // Drop a new raindrop if needed.
        if state.last_drop.elapsed() > state.drop_speed {
            let element = match state.elements.iter_mut().find(|e| !e.active) {
                Some(element) => element,
                None => {
                    state.elements.push(Element::default());
                    state.elements.last_mut().unwrap()
                }
            };

            element.init(rng, area, self.namelist);
            element.active = true;

            state.last_drop = Instant::now();
        }

        // Draw all active raindrops.
        for element in state.elements.iter_mut().filter(|e| e.active) {
            // If out of sight, deactivate the element.
            if element.pos.1 as usize
                >= area.bottom() as usize + element.name.len() + element.trail.len()
                || element.pos.0 >= area.right()
            {
                element.active = false;
                continue;
            }

            // Draw the name first.
            for (i, c) in element.name.chars().rev().enumerate() {
                if let Some(pos) = element.pos.1.checked_sub(i as u16) {
                    if pos < area.bottom() {
                        buf.get_mut(element.pos.0, pos)
                            .set_style(
                                Style::default()
                                    .fg(Color::Indexed(self.color))
                                    .bg(Color::Indexed(self.color - 24))
                                    .add_modifier(Modifier::BOLD),
                            )
                            .set_char(c);
                    }
                }
            }

            // Then the trail.
            for (i, c) in element.trail.iter().enumerate() {
                if let Some(pos) = element.pos.1.checked_sub((element.name.len() + i) as u16) {
                    if pos < area.bottom() {
                        buf.get_mut(element.pos.0, pos)
                            .set_fg(Color::Indexed(if i < element.trail.len() / 2 {
                                self.color - 12
                            } else {
                                self.color - 24
                            }))
                            .set_char(*c);
                    }
                }
            }

            // Move the element forward.
            element.pos.1 += 1;
        }
    }
}
