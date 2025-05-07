//! This module handles input events from the user.
//! It defines the `Key` enum to represent different keys
//! and provides methods to convert byte sequences
//! to `Key` values.

/// The `Key` enum represents different keys
/// that can be pressed on the keyboard.
/// It includes letters, arrow keys,
/// space, enter, and backspace.
///
/// Some keys require a specific byte sequence
/// to be recognized, such as arrow keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Space,
    Enter,
    Backspace,
}

impl Key {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            b'a' => Key::A,
            b'b' => Key::B,
            b'c' => Key::C,
            b'd' => Key::D,
            b'e' => Key::E,
            b'f' => Key::F,
            b'g' => Key::G,
            b'h' => Key::H,
            b'i' => Key::I,
            b'j' => Key::J,
            b'k' => Key::K,
            b'l' => Key::L,
            b'm' => Key::M,
            b'n' => Key::N,
            b'o' => Key::O,
            b'p' => Key::P,
            b'q' => Key::Q,
            b'r' => Key::R,
            b's' => Key::S,
            b't' => Key::T,
            b'u' => Key::U,
            b'v' => Key::V,
            b'w' => Key::W,
            b'x' => Key::X,
            b'y' => Key::Y,
            b'z' => Key::Z,
            b' ' => Key::Space,
            b'\n' => Key::Enter,
            b'\x7f' => Key::Backspace,

            _ => unreachable!(),
        }
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        match buf {
            [0x1b, 0x5b, 0x41] => Some(Key::ArrowUp),
            [0x1b, 0x5b, 0x42] => Some(Key::ArrowDown),
            [0x1b, 0x5b, 0x43] => Some(Key::ArrowRight),
            [0x1b, 0x5b, 0x44] => Some(Key::ArrowLeft),
            [byte] => match byte {
                b'a'..=b'z' => Some(Key::from_byte(*byte)),
                _ => None,
            },
            _ => None,
        }
    }
}
