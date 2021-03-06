//! ASCII-Art for several characters.
//!
//! Constants in this module contain arrays or slices of arrays with square representations of
//! characters. For example an array with length `100` describes a `10x10` grid, with length `25`
//! describes a `5x5` grid.
//!
//! The values of each array are either `1` or `0` describing whether a value should be set at that
//! position to form the shape of a character.

/// Digits from 0 to 9 described as 10x10 arrays.
#[rustfmt::skip]
pub const DIGITS: &[[u8; 100]] = &[
    [
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
    ],
    [
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
    ],
    [
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,0,0,0,0,0,0,0,
        1,1,1,0,0,0,0,0,0,0,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
    ],
    [
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
    ],
    [
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
    ],
    [
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,0,0,0,0,0,0,0,
        1,1,1,0,0,0,0,0,0,0,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
    ],
    [
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,0,0,0,0,0,0,0,
        1,1,1,0,0,0,0,0,0,0,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
    ],
    [
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
    ],
    [
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
    ],
    [
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
        0,0,0,0,0,0,0,1,1,1,
    ],
];

/// The semicolon character as a 10x10 array.
#[rustfmt::skip]
pub const SEMICOLON: [u8; 100] = [
    0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,
    0,0,0,1,1,1,1,0,0,0,
    0,0,0,1,1,1,1,0,0,0,
    0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,
    0,0,0,1,1,1,1,0,0,0,
    0,0,0,1,1,1,1,0,0,0,
    0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,
];
