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
    namelist: &'a [String],
}

impl<'a> Rain<'a> {
    pub const fn new(_color: u8, namelist: &'a [String]) -> Self {
        Self { namelist }
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

    fn update_active(&mut self, area: Rect) -> bool {
        if self.pos.1 as usize >= area.bottom() as usize + self.name.len() + self.trail.len()
            || self.pos.0 >= area.right()
        {
            self.active = false;
        }

        self.active
    }

    fn draw_name(&self, area: Rect, buf: &mut Buffer) {
        for (i, c) in self.name.chars().rev().enumerate() {
            if let Some(pos) = self.pos.1.checked_sub(i as u16) {
                if pos < area.bottom() {
                    buf.get_mut(self.pos.0, pos)
                        .set_style(
                            Style::default()
                                .fg(Color::Indexed(47))
                                .bg(Color::Indexed(23))
                                .add_modifier(Modifier::BOLD),
                        )
                        .set_char(c);
                }
            }
        }
    }

    fn draw_tail(&self, area: Rect, buf: &mut Buffer) {
        for (i, c) in self.trail.iter().enumerate() {
            if let Some(pos) = self.pos.1.checked_sub((self.name.len() + i) as u16) {
                if pos < area.bottom() {
                    buf.get_mut(self.pos.0, pos)
                        .set_fg(Color::Indexed(if i < self.trail.len() / 2 {
                            35
                        } else {
                            23
                        }))
                        .set_char(*c);
                }
            }
        }
    }

    fn step(&mut self) {
        self.pos.1 += 1;
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

fn random_katakana(rng: &mut impl Rng) -> char {
    ('\u{ff66}'..='\u{ff9d}').choose(rng).unwrap()
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
            if !element.update_active(area) {
                continue;
            }

            element.draw_name(area, buf);
            element.draw_tail(area, buf);

            element.step();
        }
    }
}

#[derive(Default)]
pub struct KanaBorder<'a> {
    title: Option<&'a str>,
}

#[derive(Default)]
pub struct KanaBorderState {
    chars: Vec<char>,
}

impl<'a> KanaBorder<'a> {
    pub const fn title(self, title: &'a str) -> Self {
        Self { title: Some(title) }
    }

    fn draw_title(&self, area: Rect, buf: &mut Buffer) {
        if let Some(title) = self.title {
            let pos = Rect::new(
                area.x + area.width / 2 - title.len() as u16 + 2,
                area.y,
                title.len() as u16,
                1,
            );
            buf.get_mut(pos.left() - 1, pos.top()).reset();
            buf.set_string(
                pos.left(),
                pos.top(),
                title,
                Style::default()
                    .fg(Color::Indexed(47))
                    .add_modifier(Modifier::BOLD),
            );
            buf.get_mut(pos.right(), pos.top()).reset();
        }
    }
}

impl<'a> StatefulWidget for KanaBorder<'a> {
    type State = KanaBorderState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let rng = &mut rand::thread_rng();

        (area.left()..area.right())
            .map(|x| (x, area.top()))
            .chain((area.left()..area.right()).map(|x| (x, area.bottom() - 1)))
            .chain((area.top() + 1..area.bottom() - 1).map(|y| (area.left(), y)))
            .chain((area.top() + 1..area.bottom() - 1).map(|y| (area.right() - 1, y)))
            .enumerate()
            .for_each(|(i, (x, y))| {
                let c = if let Some(c) = state.chars.get(i) {
                    *c
                } else {
                    let c = random_katakana(rng);
                    state.chars.push(c);
                    c
                };

                buf.get_mut(x, y).set_char(c).set_style(
                    Style::default()
                        .fg(Color::Indexed(35))
                        .add_modifier(Modifier::BOLD),
                );
            });

        self.draw_title(area, buf);
    }
}

pub struct KanaList<'a> {
    items: &'a [&'a str],
}

impl<'a> KanaList<'a> {
    pub const fn new(items: &'a [&'a str]) -> Self {
        Self { items }
    }
}

pub struct KanaListState {
    selected: usize,
    pointer: char,
    last_update: Instant,
}

impl Default for KanaListState {
    fn default() -> Self {
        Self {
            selected: 0,
            pointer: random_katakana(&mut rand::thread_rng()),
            last_update: Instant::now(),
        }
    }
}

impl KanaListState {
    pub fn next(&mut self, items: &[&str]) {
        self.selected = (self.selected + 1) % items.len();
    }

    pub fn prev(&mut self, items: &[&str]) {
        if self.selected == 0 {
            self.selected = items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }
}

const POINTER_REFRESH_TIME: Duration = Duration::from_millis(400);

impl<'a> StatefulWidget for KanaList<'a> {
    type State = KanaListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        for (i, item) in self.items.iter().enumerate() {
            let mut style = Style::default().fg(Color::Indexed(47));

            if state.last_update.elapsed() > POINTER_REFRESH_TIME {
                state.pointer = random_katakana(&mut rand::thread_rng());
                state.last_update = Instant::now();
            }

            if i == state.selected {
                style = style.add_modifier(Modifier::BOLD);
                buf.get_mut(area.left(), area.top() + i as u16)
                    .set_style(style)
                    .set_char(state.pointer);
            }

            buf.set_string(area.left() + 2, area.top() + i as u16, item, style);
        }
    }
}
