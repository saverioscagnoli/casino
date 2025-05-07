mod input;
mod mode;
mod ops;
mod traits;

use std::time::Duration;
use tokio::{
    io::{self, AsyncReadExt},
    time::timeout,
};

pub use input::*;
pub use mode::*;
pub use ops::*;
pub use traits::*;

/// Basic history structure to store command history
/// Used to replicate the behavior of the arrow keys
/// when using a common terminal
struct History {
    items: Vec<String>,
    index: usize,
}

impl History {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            index: 0,
        }
    }
}

pub struct Options {
    /// The character used to exit the console
    pub exit_signal: char,

    /// The prompt to display before each command
    pub prompt: Option<String>,

    /// The welcome message to display when the console starts
    pub welcome_message: Option<String>,

    /// Whether the command names are case sensitive
    /// If true, commands will be matched exactly
    /// If false, commands will be matched case insensitively
    pub case_sensitive: bool,

    /// The handler to use to implement custom behavior
    pub handler: Option<Box<dyn ConsoleHandler>>,

    /// The fallback command to use when no command is found
    pub default_command: Option<Box<dyn Command>>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            exit_signal: '\x03',
            prompt: None,
            welcome_message: None,
            case_sensitive: false,
            handler: None,
            default_command: None,
        }
    }
}

//// Console struct
///
/// This struct is used to create a console application
/// It allows you to add commands, set options, and run the console
///
/// # Examples
///
/// ```
/// use console::{Console, Command, ConsoleHandler};
/// use tokio::io;
///
/// #[derive(Debug)]
/// struct HelloCommand;
///
/// #[async_trait::async_trait]
/// impl Command for HelloCommand {
///    fn name(&self) -> &str {
///       "hello"
///   }
///     
///   fn description(&self) -> &str {
///      "Says hello"
///   }
///
///   async fn execute(&mut self, stdout: &mut io::Stdout, _: &[&str]) -> io::Result<()> {
///      stdout.execute(PrintLn("Hello!")).await?;
///      Ok(())
///   }
/// }
///
/// #[tokio::main]
/// async fn main() -> io::Result<()> {
///   Console::new()
///     .command(HelloCommand)
///     .prompt("-> ")
///     .welcome_message("Welcome to the console!")
///     .case_sensitive(false)
///     .exit_signal('q')
///     .run().await?;
/// }
///
pub struct Console {
    commands: Vec<Box<dyn Command>>,
    options: Options,
    history: History,
}

impl Console {
    /// Creates a new console instance
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            options: Options::default(),
            history: History::new(),
        }
    }

    /// Sets the prompt to display before each command
    ///
    /// # Examples
    /// ```
    /// use console::Console;
    ///
    /// let console = Console::new()
    ///    .prompt("-> ");
    ///
    /// ```
    #[inline]
    pub fn prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.options.prompt = Some(prompt.into());
        self
    }

    /// Sets the welcome message to display when the console starts
    ///
    /// # Examples
    /// ```
    /// use console::Console;
    ///
    /// let console = Console::new()
    ///    .welcome_message("Welcome to the console!");
    ///
    /// ```
    #[inline]
    pub fn welcome_message<S: Into<String>>(mut self, message: S) -> Self {
        self.options.welcome_message = Some(message.into());
        self
    }

    /// Sets the case sensitivity of the command names
    ///
    /// # Examples
    /// ```
    /// use console::Console;
    ///
    /// let console = Console::new()
    ///    .case_sensitive(true);
    ///
    /// ```
    #[inline]
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.options.case_sensitive = case_sensitive;
        self
    }

    /// Adds a command to the console
    ///
    /// # Examples
    /// ```
    /// use console::{Console, Command};
    ///
    /// #[derive(Debug)]
    /// struct HelloCommand;
    ///
    /// #[async_trait::async_trait]
    /// impl Command for HelloCommand {
    ///   fn name(&self) -> &str {
    ///      "hello"
    ///  }
    ///
    ///  fn description(&self) -> &str {
    ///     "Says hello"
    ///  }
    ///
    /// async fn execute(&mut self, stdout: &mut io::Stdout, _: &[&str]) -> io::Result<()> {
    ///     stdout.execute(PrintLn("Hello!")).await?;
    ///    Ok(())
    ///  }
    /// }
    ///
    /// let console = Console::new()
    ///    .command(HelloCommand);
    ///
    /// ```
    #[inline]
    pub fn command<C: Command + 'static>(mut self, command: C) -> Self {
        self.commands.push(Box::new(command));
        self
    }

    /// Sets the default command to use when no command is found
    ///
    /// # Examples
    ///
    /// ```
    /// use console::{Console, Command};
    ///
    /// #[derive(Debug)]
    /// struct DefaultCommand;
    ///
    /// #[async_trait::async_trait]
    /// impl Command for DefaultCommand {
    ///   fn name(&self) -> &str {
    ///     "default"
    ///  }
    ///
    ///  fn description(&self) -> &str {
    ///    "Default command"
    ///  }
    ///
    /// async fn execute(&mut self, stdout: &mut io::Stdout, _: &[&str]) -> io::Result<()> {
    ///     stdout.execute(PrintLn("Default command executed")).await?;
    ///    Ok(())
    ///  }
    /// }
    ///
    ///     
    /// let console = Console::new()
    ///    .default_command(DefaultCommand);
    ///
    /// ```
    #[inline]
    pub fn default_command<C: Command + 'static>(mut self, command: C) -> Self {
        self.options.default_command = Some(Box::new(command));
        self
    }

    /// Sets the character used to exit the console
    ///
    /// # Examples
    /// ```
    /// use console::Console;
    ///     
    /// let console = Console::new()
    ///   .exit_signal('q');
    ///     
    /// ```
    /// # Note
    /// The default exit signal is `Ctrl+C` (0x03)
    /// You can change it to any character you want
    #[inline]
    pub fn exit_signal(mut self, signal: char) -> Self {
        self.options.exit_signal = signal;
        self
    }

    /// Sets the handler to use to implement custom behavior
    ///     
    /// # Examples
    /// ```
    /// use console::{Console, ConsoleHandler};
    ///
    /// #[derive(Debug)]
    /// struct MyHandler;
    ///
    /// #[async_trait]
    /// impl ConsoleHandler for MyHandler {
    ///   async fn on_keypress(&mut self, stdout: &mut io::Stdout, key: Key) -> io::Result<()> {
    ///      match key {
    ///         Key::Enter => {
    ///            stdout.execute(PrintLn("Enter pressed")).await?;
    ///           }
    ///         Key::Backspace => {
    ///           stdout.execute(PrintLn("Backspace pressed")).await?;
    ///          }
    ///        _ => {}
    ///      }
    ///     Ok(())
    ///   }
    /// }
    ///
    /// let console = Console::new()
    ///   .handler(MyHandler);
    ///
    /// ```
    ///
    #[inline]
    pub fn handler<H: ConsoleHandler + 'static>(mut self, handler: H) -> Self {
        self.options.handler = Some(Box::new(handler));
        self
    }

    /// Helper function to check if the command name is case sensitive
    /// and find the command in the list of commands
    fn find_command(&mut self, name: &str) -> Option<&mut Box<dyn Command>> {
        self.commands.iter_mut().find(|cmd| {
            if self.options.case_sensitive {
                cmd.name() == name
            } else {
                cmd.name().eq_ignore_ascii_case(name)
            }
        })
    }

    /// Reads the escape sequence for arrow keys and other special keys
    /// Returns the corresponding Key enum variant
    /// If the escape sequence is not recognized, returns None
    async fn read_escape_sequence<R: AsyncReadExt + Unpin>(
        &self,
        reader: &mut R,
    ) -> io::Result<Option<Key>> {
        let mut second_byte = [0u8; 1];

        if timeout(
            Duration::from_millis(10),
            reader.read_exact(&mut second_byte),
        )
        .await
        .is_err()
        {
            return Ok(None);
        }

        if second_byte[0] != 0x5b {
            return Ok(None);
        }

        let mut third_byte = [0u8; 1];

        if timeout(
            Duration::from_millis(10),
            reader.read_exact(&mut third_byte),
        )
        .await
        .is_err()
        {
            return Ok(None);
        }

        Ok(Key::from_bytes(&[0x1b, 0x5b, third_byte[0]]))
    }

    /// Runs the console application
    /// This function will block until the console is exited.
    pub async fn run(&mut self) -> io::Result<()> {
        // Setup
        let mut stdout = io::stdout();
        let mut stdin = io::stdin();
        let mut line = String::new();

        // Disables raw mode when the program exits
        // So that the terminal is restored to its original state
        let _raw_mode_guard = RawModeGuard::new().expect("Failed to enable raw mode");

        // Print the welcome message if set
        if let Some(ref message) = self.options.welcome_message {
            stdout.execute(PrintLn(message)).await?;
        }

        // Print the prompt if set
        if let Some(ref prompt) = self.options.prompt {
            stdout.execute(Print(prompt)).await?;
        }

        loop {
            // Read the first byte from stdin
            let mut first_byte = [0u8; 1];
            stdin.read_exact(&mut first_byte).await?;

            let byte = first_byte[0];

            // Check if the byte is the start of an escape sequence
            if byte == 0x1b {
                // Read the next byte to check if it's an escape sequence
                if let Some(key) = self.read_escape_sequence(&mut stdin).await? {
                    match key {
                        Key::ArrowUp => {
                            if self.history.index > 0 {
                                self.history.index -= 1;
                                stdout.execute(ClearLine).await?;
                                stdout.execute(MoveToColumn(0)).await?;

                                if let Some(ref prompt) = self.options.prompt {
                                    stdout.execute(Print(prompt)).await?;
                                }

                                line = self.history.items[self.history.index].clone();
                                stdout.execute(Print(&line)).await?;
                            }
                        }

                        Key::ArrowDown => {
                            if self.history.index + 1 < self.history.items.len() {
                                self.history.index += 1;
                                stdout.execute(ClearLine).await?;
                                stdout.execute(MoveToColumn(0)).await?;

                                if let Some(ref prompt) = self.options.prompt {
                                    stdout.execute(Print(prompt)).await?;
                                }

                                line = self.history.items[self.history.index].clone();
                                stdout.execute(Print(&line)).await?;
                            } else {
                                self.history.index = self.history.items.len();
                                stdout.execute(ClearLine).await?;
                                stdout.execute(MoveToColumn(0)).await?;

                                if let Some(ref prompt) = self.options.prompt {
                                    stdout.execute(Print(prompt)).await?;
                                }

                                line.clear();
                            }
                        }

                        _ => {}
                    }

                    if let Some(ref mut handler) = self.options.handler {
                        handler.on_keypress(&mut stdout, key).await?;
                    }
                }
            } else {
                // Else its a normal character

                match byte {
                    0x7f => {
                        if !line.is_empty() {
                            line.pop();
                            stdout.execute(Backspace(1)).await?;
                        }

                        if let Some(ref mut handler) = self.options.handler {
                            handler.on_keypress(&mut stdout, Key::Backspace).await?;
                        }
                    }

                    b'\n' => {
                        stdout.execute(Print('\n')).await?;

                        if let Some(ref mut handler) = self.options.handler {
                            handler.on_keypress(&mut stdout, Key::Enter).await?;
                        }

                        if line.is_empty() {
                            if let Some(ref prompt) = self.options.prompt {
                                stdout.execute(Print(prompt)).await?;
                            }

                            continue;
                        }

                        let parts = line.split_whitespace().collect::<Vec<_>>();

                        if parts.is_empty() {
                            continue;
                        }

                        let name = parts[0];

                        if let Some(command) = self.find_command(name) {
                            let args = &parts[1..];
                            command.execute(&mut stdout, args).await?;
                        } else if let Some(ref mut default_command) = self.options.default_command {
                            default_command.execute(&mut stdout, &parts).await?;
                        }

                        if let Some(ref prompt) = self.options.prompt {
                            stdout.execute(Print(prompt)).await?;
                        }

                        self.history.items.push(line.clone());
                        self.history.index = self.history.items.len();
                        line.clear();
                    }

                    _ => {
                        line.push(byte as char);

                        stdout.execute(Print(byte as char)).await?;

                        let key = Key::from_byte(byte);

                        if let Some(ref mut handler) = self.options.handler {
                            handler.on_keypress(&mut stdout, key).await?;
                        }
                    }
                }
            }
        }
    }
}
