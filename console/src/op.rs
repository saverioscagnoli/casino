use crate::traits::Op;
use std::fmt::Display;

/// A wrapper for printing text to the terminal without a newline.
///
/// # Example
///
/// ```no_run
/// use console::op::Print;
/// use console::CommandExecutor;
/// use tokio::io::stdout;
///
/// #[tokio::main]
/// async fn main() -> tokio::io::Result<()> {
///     let mut stdout = stdout();
///     stdout.execute(Print("Hello, world!")).await?;
///     Ok(())
/// }
/// ```
pub struct Print<T: Display>(pub T);

unsafe impl<T: Display> Send for Print<T> {}
unsafe impl<T: Display> Sync for Print<T> {}

impl<T: Display> Op for Print<T> {
    /// Converts the contained value to a string for terminal display.
    ///
    /// # Returns
    ///
    /// A string representation of the wrapped value.
    fn ansi(&self) -> String {
        self.0.to_string()
    }
}

/// A wrapper for printing text to the terminal with a trailing newline.
///
/// # Example
///
/// ```no_run
/// use console::op::PrintLn;
/// use console::CommandExecutor;
/// use tokio::io::stdout;
///
/// #[tokio::main]
/// async fn main() -> tokio::io::Result<()> {
///     let mut stdout = stdout();
///     stdout.execute(PrintLn("Hello, world!")).await?;
///     Ok(())
/// }
/// ```
pub struct PrintLn<T: Display>(pub T);

unsafe impl<T: Display> Send for PrintLn<T> {}
unsafe impl<T: Display> Sync for PrintLn<T> {}

impl<T: Display> Op for PrintLn<T> {
    /// Converts the contained value to a string and appends a newline.
    ///
    /// # Returns
    ///
    /// A string representation of the wrapped value followed by a newline.
    fn ansi(&self) -> String {
        self.0.to_string() + "\n"
    }
}

/// Specifies the type of terminal clear operation to perform.
///
/// # Variants
///
/// * `All` - Clears the entire screen and resets cursor position
/// * `Line` - Clears only the current line
#[derive(Debug, Clone, Copy)]
pub enum ClearKind {
    /// Clears the entire terminal screen and moves cursor to top-left
    All,
    /// Clears only the current line, keeping cursor position
    Line,
}

/// A terminal operation that clears content according to the specified kind.
///
/// # Example
///
/// ```no_run
/// use console::op::{Clear, ClearKind};
/// use console::CommandExecutor;
/// use tokio::io::stdout;
///
/// #[tokio::main]
/// async fn main() -> tokio::io::Result<()> {
///     let mut stdout = stdout();
///     // Clear the entire screen
///     stdout.execute(Clear(ClearKind::All)).await?;
///     Ok(())
/// }
/// ```
pub struct Clear(pub ClearKind);

unsafe impl Send for Clear {}
unsafe impl Sync for Clear {}

impl Op for Clear {
    /// Generates the ANSI escape sequence for clearing the terminal.
    ///
    /// # Returns
    ///
    /// An ANSI escape sequence string based on the clear kind.
    fn ansi(&self) -> String {
        match self.0 {
            ClearKind::All => "\x1B[2J\x1B[H".to_string(), // Clear the entire screen and move the cursor to the top-left
            ClearKind::Line => "\x1B[2K".to_string(),      // Clear the current line
        }
    }
}

/// A terminal operation that moves the cursor to a specific position.
///
/// Takes row and column coordinates as parameters.
///
/// # Example
///
/// ```no_run
/// use console::op::MoveTo;
/// use console::CommandExecutor;
/// use tokio::io::stdout;
///
/// #[tokio::main]
/// async fn main() -> tokio::io::Result<()> {
///     let mut stdout = stdout();
///     // Move cursor to row 5, column 10
///     stdout.execute(MoveTo(5, 10)).await?;
///     Ok(())
/// }
/// ```
pub struct MoveTo(pub u16, pub u16);

unsafe impl Send for MoveTo {}
unsafe impl Sync for MoveTo {}

impl Op for MoveTo {
    /// Generates the ANSI escape sequence for moving the cursor.
    ///
    /// # Returns
    ///
    /// An ANSI escape sequence string for cursor positioning.
    fn ansi(&self) -> String {
        format!("\x1B[{};{}H", self.0, self.1)
    }
}
