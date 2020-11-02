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