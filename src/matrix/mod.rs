//!

use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use rand::{distributions::Uniform, prelude::*};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{StatefulWidget, Widget},
};

use crate::RectExt;

mod asciiart;

/// The iconic Matrix rain widget drawing rain drops rendered as random characters. The tip of the
/// rain drops contains random names for the namelist and tails are randomized characters.
#[derive(Copy, Clone)]
pub struct Rain<'a> {
    /// List of names to pick from for new rain drops.
    namelist: &'a [String],
    /// Speed at which rain drops move down in the scene or _fall_.
    update_speed: Duration,
    /// Speed at which new drops are added to the scene.
    drop_speed: Duration,
}

impl<'a> Rain<'a> {
    /// Create a new Matrix rain with that picks random names from the given list to be drawn at the
    /// tip of rain drops. Update speed defines how fast rain drops fall and drop speed defines
    /// often new drops start falling from the top.
    pub const fn new(namelist: &'a [String], update_speed: Duration, drop_speed: Duration) -> Self {
        Self {
            namelist,
            update_speed,
            drop_speed,
        }
    }
}

/// State for the [`Rain`] widget.
pub struct RainState<'a> {
    /// Pool of rain drops either active or not. Inactive drops can be reused as they left the
    /// drawing area already.
    raindrops: Vec<RainDrop<'a>>,
    /// Last time a new drop was added to the scene.
    last_drop: Instant,
    /// Last time all drops' position was updated.
    last_update: Instant,
}

impl<'a> RainState<'a> {
    /// Create a new empty rain state.
    pub fn new() -> Self {
        Self {
            raindrops: Vec::new(),
            last_drop: Instant::now(),
            last_update: Instant::now(),
        }
    }
}

/// A single Matrix rain drop as part of the [`RainState`].
#[derive(Default)]
struct RainDrop<'a> {
    /// Name to draw at the tip.
    name: &'a str,
    /// Tail that's drawn directly behind the name.
    trail: VecDeque<char>,
    /// Current position within the terminal.
    pos: (u16, u16),
    /// Flag to tell whether a drop is still visible. Allows to keep a pool of drops for reuse.
    active: bool,
}

impl<'a> RainDrop<'a> {
    /// Initialize a new rain drop with a random name from the given list, a tail of random
    /// characters and a random horizontal position within the given area.
    fn init(&mut self, rng: &mut impl Rng, area: Rect, namelist: &'a [String]) {
        self.name = namelist.choose(rng).unwrap();
        self.trail = (0..rng.sample(Uniform::new_inclusive(self.name.len(), self.name.len() * 2)))
            .map(|_| random_char(rng))
            .collect();
        self.pos = (rng.gen::<u16>() % area.right(), 0);
    }

    /// Check whether this drop is still within the given area and turn it inactive if it's not.
    fn update_active(&mut self, area: Rect) -> bool {
        if self.pos.1 as usize >= area.bottom() as usize + self.name.len() + self.trail.len()
            || self.pos.0 >= area.right()
        {
            self.active = false;
        }

        self.active
    }

    /// Draw the name vertically at the tip of the rain drop.
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

    /// Draw the tail of the drop directly behind the name.
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

    /// Move the drop one step forward. This simply moves it one line down.
    fn step(&mut self, rng: &mut impl Rng) {
        self.pos.1 += 1;
        self.trail.push_front(random_char(rng));
        self.trail.pop_back();
    }
}

/// Generate a random character which is an alphabetic uppercase letter (A-Z), a digit (0-9), or a
/// Half-Width Katakana, with a high chance for Katakanas.
#[inline]
fn random_char(rng: &mut impl Rng) -> char {
    match rng.next_u32() % 5 {
        0 => ('A'..='Z').choose(rng).unwrap(),
        1 => random_digit(rng),
        _ => random_katakana(rng),
    }
}

/// Generate a random digit (0-9) character.
#[inline]
fn random_digit(rng: &mut impl Rng) -> char {
    ('0'..='9').choose(rng).unwrap()
}

/// Generate a random Half-Width Katakana character.
#[inline]
fn random_katakana(rng: &mut impl Rng) -> char {
    ('\u{ff66}'..='\u{ff9d}').choose(rng).unwrap()
}

impl<'a> StatefulWidget for Rain<'a> {
    type State = RainState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let rng = &mut rand::thread_rng();

        // Drop a new raindrop if needed.
        if state.last_drop.elapsed() > self.drop_speed {
            let element = match state.raindrops.iter_mut().find(|e| !e.active) {
                Some(element) => element,
                None => {
                    state.raindrops.push(RainDrop::default());
                    state.raindrops.last_mut().unwrap()
                }
            };

            element.init(rng, area, self.namelist);
            element.active = true;

            state.last_drop = Instant::now();
        }

        let step = if state.last_update.elapsed() > self.update_speed {
            state.last_update = Instant::now();
            true
        } else {
            false
        };

        // Draw all active raindrops.
        for element in state.raindrops.iter_mut().filter(|e| e.active) {
            if !element.update_active(area) {
                continue;
            }

            element.draw_name(area, buf);
            element.draw_tail(area, buf);

            if step {
                element.step(rng);
            }
        }
    }
}

/// Katakana border widget that draws a border around an area with random Half-Width Katakanas. The
/// characters are replaced randomly and an optional title can be set.
///
/// # Example output
///
/// ```txt
/// ｹﾑｹｫﾇｾﾃﾂｸﾜｧﾑﾅｵﾐﾎｲ TITLE ﾘｵｦｰﾌﾒﾏﾇｸｶﾐｪﾁﾙﾜﾁﾁﾎ
/// ｯ                                  ｬ
/// ｴ  Hello World!                    ﾒ
/// ﾆ                                  ﾖ
/// ﾛｮﾗｧｾﾂｨﾐﾜﾉｽﾚﾒｪｺﾆｿｰﾍﾏｵﾝｷﾈｼﾇｵﾜﾗﾉｵｶｲｽｹｵｪｮｬﾔｯﾇｱ
/// ```
#[derive(Default)]
pub struct KanaBorder<'a> {
    /// Optional title drawn at the top corner.
    title: Option<&'a str>,
}

/// State for the [`KanaBorder`] widget. This state can be shared by multiple border instances as it
/// only holds a buffer to keep track of the current used chars in the border.
#[derive(Default)]
pub struct KanaBorderState {
    /// Buffer of chars that hold the random elements to draw the border.
    chars: Vec<char>,
}

impl<'a> KanaBorder<'a> {
    /// Set a title to be shown at the center top side border.
    pub const fn title(self, title: &'a str) -> Self {
        Self { title: Some(title) }
    }

    /// Draw a title if it's set in the middle of the top border. A space is but before and after
    /// the title to make it more readable.
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
                let mut c = if let Some(c) = state.chars.get(i) {
                    *c
                } else {
                    let c = random_katakana(rng);
                    state.chars.push(c);
                    c
                };

                if rng.next_u32() % 100 == 0 {
                    c = random_katakana(rng);
                    state.chars[i] = c;
                }

                buf.get_mut(x, y).set_char(c).set_style(
                    Style::default()
                        .fg(Color::Indexed(35))
                        .add_modifier(Modifier::BOLD),
                );
            });

        self.draw_title(area, buf);
    }
}

/// List widget which can select a single item. The current item is indicated by a single Katakana
/// character that changes randomly.
///
/// # Example output
///
/// ```txt
/// ｦ Countdown
/// ```
pub struct KanaList<'a> {
    items: &'a [&'a str],
}

impl<'a> KanaList<'a> {
    /// Speed at which the pointer character is exchanged for a new random character.
    const POINTER_REFRESH_TIME: Duration = Duration::from_millis(400);

    /// Create a new list widget with the given slice of items to display.
    pub const fn new(items: &'a [&'a str]) -> Self {
        Self { items }
    }
}

/// State for the [`KanaList`] widget.
pub struct KanaListState {
    /// Index of the currently selected item.
    selected: usize,
    /// Random Katakana character to point at the current item.
    pointer: char,
    /// Last time the pointer has been updated.
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
    /// Currently selected list item.
    pub const fn selected(&self) -> usize {
        self.selected
    }

    /// Select the next item in the list or jump to the first item if currently at the bottom.
    pub fn next(&mut self, items: &[&str]) {
        self.selected = (self.selected + 1) % items.len();
    }

    /// Select the previous item in the list or jump to the last item if currently at the top.
    pub fn prev(&mut self, items: &[&str]) {
        if self.selected == 0 {
            self.selected = items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }
}

impl<'a> StatefulWidget for KanaList<'a> {
    type State = KanaListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        for (i, item) in self.items.iter().enumerate() {
            let mut style = Style::default().fg(Color::Indexed(47));

            if state.last_update.elapsed() > Self::POINTER_REFRESH_TIME {
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

/// Countdown widget that draws the minutes and seconds of the duration field as ASCII-Art on the
/// buffer. The content will be centered within the area.
///
/// # Example output
///
/// ```txt
/// 9486281943 2346536193            821    111 7498274576
/// 5730356901 1675673564            134    323 3904853295
/// 358    483        902    7349    394    876        345
/// 263    135        421    9247    348    473        224
/// 567    053 8962387539            2980435739        112
/// 023    346 9184573295            3298563097        977
/// 043    245 654           3772           138        453
/// 204    987 927           1173           234        098
/// 0991356113 0582759482                   847        245
/// 1343533465 9348672928                   009        551
/// ```
pub struct Countdown {
    /// Current duration to draw. Only minutes and seconds are considered.
    pub duration: Duration,
}

impl Countdown {
    /// Draw the shape of a single character described by the symbol array, where a non-zero value
    /// means to draw a random digit and a zero value means not to draw anything at the position.
    ///
    /// The background color of each drawn cell has a chance to be a brighter color to generate a
    /// flicker effect.
    fn draw_shape(area: Rect, buf: &mut Buffer, rng: &mut impl Rng, symbol: [u8; 100]) {
        for (y, row) in symbol.chunks_exact(10).enumerate() {
            for (x, set) in row.iter().enumerate() {
                if *set != 0 {
                    let cell = buf.get_mut(area.x + x as u16, area.y + y as u16);
                    cell.reset();
                    cell.set_bg(Color::Indexed(if rng.next_u32() % 5 == 0 {
                        35
                    } else {
                        23
                    }))
                    .set_fg(Color::Indexed(47))
                    .set_char(random_digit(rng));
                }
            }
        }
    }
}

impl Widget for Countdown {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rng = &mut rand::thread_rng();
        let mut r = Rect::new(0, 0, 54, 10).center_in(area);

        let secs = self.duration.as_secs() as usize;

        Self::draw_shape(r, buf, rng, self::asciiart::DIGITS[secs / 60 / 10]);
        r.x += 11;
        Self::draw_shape(r, buf, rng, self::asciiart::DIGITS[secs / 60 % 10]);
        r.x += 11;
        Self::draw_shape(r, buf, rng, self::asciiart::SEMICOLON);
        r.x += 11;
        Self::draw_shape(r, buf, rng, self::asciiart::DIGITS[secs % 60 / 10]);
        r.x += 11;
        Self::draw_shape(r, buf, rng, self::asciiart::DIGITS[secs % 10]);
    }
}

pub struct KanaBackground {
    update_speed: Duration,
}

pub struct KanaBackgroundState {
    chars: Vec<(char, u16, u16)>,
    last_update: Instant,
}

impl Default for KanaBackgroundState {
    fn default() -> Self {
        Self {
            chars: Vec::new(),
            last_update: Instant::now(),
        }
    }
}

impl KanaBackground {
    pub const fn new(update_speed: Duration) -> Self {
        Self { update_speed }
    }

    fn new_random(rng: &mut impl Rng, area: Rect) -> (char, u16, u16) {
        (
            random_katakana(&mut rand::thread_rng()),
            rng.gen::<u16>() % area.right(),
            rng.gen::<u16>() % area.bottom(),
        )
    }
}

impl StatefulWidget for KanaBackground {
    type State = KanaBackgroundState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let rng = &mut rand::thread_rng();
        let amount = area.width as usize * area.height as usize / 20;

        if state.chars.len() != amount {
            state.chars.clear();
            state
                .chars
                .resize_with(amount, || Self::new_random(rng, area));
        }

        if state.last_update.elapsed() > self.update_speed {
            for _ in 0..amount / 20 {
                if let Some(c) = state.chars.choose_mut(rng) {
                    let new = Self::new_random(rng, area);
                    *c = new;
                }
            }
            state.last_update = Instant::now();
        }

        for (c, x, y) in state
            .chars
            .iter()
            .copied()
            .filter(|v| area.contains((v.1, v.2)))
        {
            buf.get_mut(x, y).set_char(c).set_style(
                Style::reset()
                    .fg(Color::Indexed(22))
                    .add_modifier(Modifier::DIM),
            );
        }
    }
}
