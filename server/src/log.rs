use std::{
    io::Write,
    sync::atomic::{AtomicBool, Ordering},
};

use traccia::{Colorize, Hook, LogLevel, TargetId, error};

use crate::consts::CONSOLE_PROMPT;

pub struct CustomFormatter;

impl traccia::Formatter for CustomFormatter {
    fn format(&self, record: &traccia::Record) -> String {
        let level = record.level.default_coloring().to_lowercase();
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let gray = traccia::Color::RGB(128, 128, 128);

        match record.level {
            LogLevel::Error | LogLevel::Fatal => {
                format!(
                    "[{} {} @ {}:{} {}] {}",
                    timestamp.to_string().color(gray),
                    level,
                    record.file.unwrap_or("unknown location"),
                    record.line.unwrap_or(0),
                    record.target.color(gray),
                    record.message,
                )
            }

            _ => {
                format!(
                    "[{} {} {}] {}",
                    timestamp.to_string().color(gray),
                    level,
                    record.target.color(gray),
                    record.message
                )
            }
        }
    }
}

/// Returns the log level for debugging / release
///
/// Used when no --level argument is passed
pub fn default_level() -> LogLevel {
    if cfg!(debug_assertions) {
        LogLevel::Debug
    } else {
        LogLevel::Info
    }
}

/// Returns the logging config
pub fn config(level: LogLevel) -> traccia::Config {
    traccia::Config {
        level,
        format: Some(Box::new(CustomFormatter)),
        ..Default::default()
    }
}

/// Sets the logger hooks to properly display the console prompt
/// when logs appear
pub fn set_hooks() {
    static FIRST: AtomicBool = AtomicBool::new(true);

    traccia::set_hook(Hook::BeforeLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            print!("\r\x1B[K"); // \r moves cursor to start of line, \x1B[K clears to end of line

            if let Err(e) = std::io::stdout().flush() {
                error!("Failed to flush stdout: {}", e);
            }
        }
    })));

    traccia::set_hook(Hook::AfterLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            if !FIRST.swap(false, Ordering::Relaxed) {
                print!("{}", CONSOLE_PROMPT);
            }

            if let Err(e) = std::io::stdout().flush() {
                error!("Failed to flush stdout: {}", e);
            }
        }
    })));
}
