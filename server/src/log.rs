use std::io::Write;

use traccia::{Color, Colorize, Hook, LogLevel, TargetId};

fn default_level() -> LogLevel {
    if cfg!(debug_assertions) {
        LogLevel::Debug
    } else {
        LogLevel::Info
    }
}

struct Formatter;

impl traccia::Formatter for Formatter {
    fn format(&self, record: &traccia::Record) -> String {
        format!(
            "{} {}: {}",
            chrono::Local::now()
                .format("%Y/%m/%d %H:%M:%S")
                .to_string()
                .color(Color::RGB(128, 128, 128)),
            record.level.default_coloring().to_lowercase(),
            record.message,
        )
    }
}

pub fn setup_logging() {
    traccia::set_hook(Hook::BeforeLog(Box::new(|_, target_id| {
        if let TargetId::Console(_) = target_id {
            // Clear the prompt line
            print!("\x1b[2K\x1b[1G");
            // Move the cursor to the beginning of the line
            print!("\x1b[1G");
        }
    })));

    traccia::set_hook(Hook::AfterLog(Box::new(|_, target_id| {
        if let TargetId::Console(_) = target_id {
            // Move the cursor to the beginning of the line
            print!("\x1b[1G");
            // Print the prompt
            print!("> ");
            // Flush the output
            if let Err(e) = std::io::stdout().flush() {
                eprint!("Error flushing stdout: {}", e);
            }
        }
    })));

    traccia::init_with_config(traccia::Config {
        level: default_level(),
        format: Some(Box::new(Formatter)),
        ..Default::default()
    });
}
