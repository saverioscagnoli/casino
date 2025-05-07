use std::fmt::Display;

use crate::traits::Op;

pub struct Print<T: Display>(pub T);

impl<T: Display> Op for Print<T> {
    fn ansi(&self) -> String {
        self.0.to_string()
    }
}

pub struct PrintLn<T: Display>(pub T);

impl<T: Display> Op for PrintLn<T> {
    fn ansi(&self) -> String {
        format!("{}\n", self.0)
    }
}

pub struct Clear;

impl Op for Clear {
    fn ansi(&self) -> String {
        "\x1b[2J\x1b[H".to_string()
    }
}

pub struct ClearLine;

impl Op for ClearLine {
    fn ansi(&self) -> String {
        "\x1b[2K".to_string()
    }
}

pub struct Backspace(pub usize);

impl Op for Backspace {
    fn ansi(&self) -> String {
        let spaces = " ".repeat(self.0);
        format!("\x1b[{}D{}\x1b[{}D", self.0, spaces, self.0)
    }
}

pub struct MoveUp(pub usize);

impl Op for MoveUp {
    fn ansi(&self) -> String {
        format!("\x1b[{}A", self.0)
    }
}

pub struct MoveDown(pub usize);

impl Op for MoveDown {
    fn ansi(&self) -> String {
        format!("\x1b[{}B", self.0)
    }
}

pub struct MoveLeft(pub usize);

impl Op for MoveLeft {
    fn ansi(&self) -> String {
        format!("\x1b[{}D", self.0)
    }
}

pub struct MoveRight(pub usize);

impl Op for MoveRight {
    fn ansi(&self) -> String {
        format!("\x1b[{}C", self.0)
    }
}

pub struct MoveTo(pub usize, pub usize);

impl Op for MoveTo {
    fn ansi(&self) -> String {
        format!("\x1b[{};{}H", self.0, self.1)
    }
}

pub struct MoveToColumn(pub usize);

impl Op for MoveToColumn {
    fn ansi(&self) -> String {
        format!("\x1b[{}G", self.0)
    }
}
