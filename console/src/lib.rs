mod traits;
mod util;

pub mod op;

pub use async_trait::async_trait;
pub use traits::{Command, CommandExecutor};
use util::{BoxAsyncFn, RawModeGuard, box_async_fn};
pub use util::{disable_raw_mode, enable_raw_mode};

use op::Print;
use tokio::io::AsyncReadExt;

/// A terminal console implementation that processes input commands.
///
/// This console creates an infinite loop that reads user input
/// and executes commands based on that input.
///
/// # Example
///
/// ```no_run
/// use console::Console;
/// use console::Command;
///
/// #[tokio::main]
/// async fn main() -> tokio::io::Result<()> {
///     Console::new()
///         .prompt("> ")
///         .command(MyCustomCommand)
///         .run()
///         .await
/// }
/// ```
pub struct Console {
    prompt: Option<String>,
    exit_signal: char,
    commands: Vec<Box<dyn Command>>,
    default_callback: Option<BoxAsyncFn>,
    case_sensitive: bool,
}

impl Console {
    /// Creates a new console instance with default settings.
    ///
    /// The default console has no prompt, uses CTRL+C as exit signal,
    /// and has no registered commands.
    pub fn new() -> Self {
        Self {
            prompt: None,
            exit_signal: '\x03',
            commands: Vec::new(),
            default_callback: None,
            case_sensitive: true,
        }
    }

    /// Sets the prompt string for the console.
    ///
    /// The prompt will be displayed before each input line.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use console::Console;
    ///
    /// let console = Console::new()
    ///     .prompt("user@localhost:~$ ");
    /// ```
    pub fn prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Changes the character that signals console termination.
    ///
    /// By default, this is set to '\x03' (CTRL+C).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use console::Console;
    ///
    /// // Set 'q' as the exit character
    /// let console = Console::new()
    ///     .exit_signal('q');
    /// ```
    pub fn exit_signal<C: Into<char>>(mut self, ch: C) -> Self {
        self.exit_signal = ch.into();
        self
    }

    /// Registers a command handler to the console.
    ///
    /// Commands are identified by their name and executed when the user
    /// types that name at the console.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use console::{Console, Command};
    ///
    /// struct ClearCommand;
    /// // Implement Command trait for ClearCommand...
    ///
    /// let console = Console::new()
    ///     .command(ClearCommand);
    /// ```
    pub fn command<C: Command + 'static>(mut self, command: C) -> Self {
        self.commands.push(Box::new(command));
        self
    }

    /// Sets a default callback to execute when no specific command is entered.
    ///
    /// This function allows you to define a fallback behavior that runs when the user
    /// presses Enter without typing a command. This is useful for providing help messages,
    /// status summaries, or other default responses.
    ///
    /// # Arguments
    ///
    /// * `callback` - A closure or function that takes a mutable reference to `tokio::io::Stdout`
    ///   and the bad command name, allowing you to define custom behavior.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use console::Console;
    /// use tokio::io::{self, Stdout};
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let console = Console::new()
    ///         .default_callback(|stdout: &mut Stdout, input: &str| {
    ///             use std::io::Write;
    ///             writeln!(stdout, "No command entered. You typed: '{}'", input).unwrap();
    ///         });
    ///
    ///     // continue running your console loop...
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - This does not override command-specific behavior; it only applies when no command is matched.
    /// - The callback is stored in a boxed closure with `'static` lifetime, so it can capture environment variables or state if needed.

    pub fn default_callback<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn(tokio::io::Stdout, String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = tokio::io::Result<()>> + Send + 'static,
    {
        self.default_callback = Some(box_async_fn(callback));
        self
    }

    /// Sets whether command matching should be case-sensitive.
    ///
    /// By default, command matching is case-sensitive.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use console::Console;
    ///
    /// // Make command matching case-insensitive
    /// let console = Console::new()
    ///     .case_sensitive(false);
    /// ```
    pub fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = value;
        self
    }

    /// Searches for a command by its name.
    ///
    /// Respects the case_sensitive setting when matching.
    fn find_command(&mut self, name: &str) -> Option<&mut Box<dyn Command>> {
        if self.case_sensitive {
            self.commands.iter_mut().find(|c| c.name() == name)
        } else {
            self.commands
                .iter_mut()
                .find(|c| c.name().eq_ignore_ascii_case(name))
        }
    }

    /// Starts the console input loop.
    ///
    /// This method blocks until the exit signal is received.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use console::Console;
    ///
    /// #[tokio::main]
    /// async fn main() -> tokio::io::Result<()> {
    ///     Console::new()
    ///         .prompt("> ")
    ///         .run()
    ///         .await
    /// }
    /// ```
    pub async fn run(mut self) -> tokio::io::Result<()> {
        let mut stdout = tokio::io::stdout();
        let mut stdin = tokio::io::stdin();
        let mut line = String::new(); // Buffer to store user input

        let _mode_guard = RawModeGuard::new()?;

        if let Some(ref prompt) = self.prompt {
            stdout.execute(Print(prompt)).await?;
        }

        loop {
            // Read one character at a time
            let mut buf = [0u8; 1];

            stdin.read_exact(&mut buf).await?;

            let ch: char = buf[0].into();

            match ch {
                '\n' => {
                    stdout.execute(Print("\n")).await?;

                    let mut parts = line.split_whitespace();
                    let command_name = parts.next();

                    // Search for a valid command
                    // If found, parse the args and execute it
                    if let Some(name) = command_name {
                        if let Some(command) = self.find_command(&name) {
                            command
                                .execute(&mut stdout, parts.collect::<Vec<_>>())
                                .await?;
                        } else if let Some(ref callback) = self.default_callback {
                            callback(tokio::io::stdout(), name.to_string()).await?;
                        }
                    }

                    // Clear input buffer
                    line.clear();

                    if let Some(ref prompt) = self.prompt {
                        stdout.execute(Print(prompt)).await?;
                    }
                }

                // Handle backspace,
                // pop the buffer and cancel last character
                // Move cursor back, overwrite with space, and move back again
                '\x08' | '\x7f' => {
                    if !line.is_empty() {
                        line.pop();
                        stdout.execute(Print("\x08 \x08")).await?;
                    }
                }

                // Handle exit signal, break the loop when sent
                s if s == self.exit_signal => {
                    stdout.execute(Print("\n")).await?;
                    break;
                }

                // Regular line characters
                // This displays the line as being typed
                _ => {
                    // Append the character to the buffer
                    line.push(ch);
                    stdout.execute(Print(ch)).await?;
                }
            }
        }

        // Here mode_guard will be dropped and will disable raw mode
        Ok(())
    }
}
