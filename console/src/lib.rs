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
    pub exit_signal: char,
    pub prompt: Option<Box<dyn Fn() -> String>>,
    pub welcome_message: Option<Box<dyn Fn() -> String>>,
    pub case_sensitive: bool,
    pub handler: Option<Box<dyn ConsoleHandler>>,
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

pub struct Console {
    commands: Vec<Box<dyn Command>>,
    options: Options,
    history: History,
}

impl Console {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            options: Options::default(),
            history: History::new(),
        }
    }

    #[inline]
    pub fn prompt<F: Fn() -> String + 'static>(mut self, prompt: F) -> Self {
        self.options.prompt = Some(Box::new(prompt));
        self
    }

    #[inline]
    pub fn welcome_message<F: Fn() -> String + 'static>(mut self, message: F) -> Self {
        self.options.welcome_message = Some(Box::new(message));
        self
    }

    #[inline]
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.options.case_sensitive = case_sensitive;
        self
    }

    #[inline]
    pub fn command<C: Command + 'static>(mut self, command: C) -> Self {
        self.commands.push(Box::new(command));
        self
    }

    #[inline]
    pub fn default_command<C: Command + 'static>(mut self, command: C) -> Self {
        self.options.default_command = Some(Box::new(command));
        self
    }

    #[inline]
    pub fn exit_signal(mut self, signal: char) -> Self {
        self.options.exit_signal = signal;
        self
    }

    #[inline]
    pub fn handler<H: ConsoleHandler + 'static>(mut self, handler: H) -> Self {
        self.options.handler = Some(Box::new(handler));
        self
    }

    fn find_command(&mut self, name: &str) -> Option<&mut Box<dyn Command>> {
        self.commands.iter_mut().find(|cmd| {
            if self.options.case_sensitive {
                cmd.name() == name
            } else {
                cmd.name().eq_ignore_ascii_case(name)
            }
        })
    }

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

    pub async fn run(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let mut stdin = io::stdin();
        let mut line = String::new();

        let _raw_mode_guard = RawModeGuard::new().expect("Failed to enable raw mode");

        if let Some(ref message) = self.options.welcome_message {
            stdout.execute(PrintLn(message())).await?;
        }

        if let Some(ref prompt) = self.options.prompt {
            stdout.execute(Print(prompt())).await?;
        }

        loop {
            let mut first_byte = [0u8; 1];
            stdin.read_exact(&mut first_byte).await?;

            let byte = first_byte[0];

            if byte == 0x1b {
                if let Some(key) = self.read_escape_sequence(&mut stdin).await? {
                    match key {
                        Key::ArrowUp => {
                            if self.history.index > 0 {
                                self.history.index -= 1;
                                stdout.execute(ClearLine).await?;
                                stdout.execute(MoveToColumn(0)).await?;

                                if let Some(ref prompt) = self.options.prompt {
                                    stdout.execute(Print(prompt())).await?;
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
                                    stdout.execute(Print(prompt())).await?;
                                }

                                line = self.history.items[self.history.index].clone();
                                stdout.execute(Print(&line)).await?;
                            } else {
                                self.history.index = self.history.items.len();
                                stdout.execute(ClearLine).await?;
                                stdout.execute(MoveToColumn(0)).await?;

                                if let Some(ref prompt) = self.options.prompt {
                                    stdout.execute(Print(prompt())).await?;
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
                match byte {
                    b'a'..=b'z' | b' ' => {
                        line.push(byte as char);

                        stdout.execute(Print(byte as char)).await?;

                        let key = Key::from_byte(byte);

                        if let Some(ref mut handler) = self.options.handler {
                            handler.on_keypress(&mut stdout, key).await?;
                        }
                    }

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
                                stdout.execute(Print(prompt())).await?;
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
                            stdout.execute(Print(prompt())).await?;
                        }

                        self.history.items.push(line.clone());
                        self.history.index = self.history.items.len();
                        line.clear();
                    }

                    _ => {}
                }
            }
        }
    }
}
