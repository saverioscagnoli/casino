use std::os::unix::io::AsRawFd;
use termios::*;

/// Enable raw mode for unix terminals
///
/// Note: Not supported on Windows
pub fn enable_raw_mode() -> tokio::io::Result<Termios> {
    let stdin_fd = tokio::io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(stdin_fd)?;
    let original = termios.clone();

    termios.c_lflag &= !(ICANON | ECHO); // disable canonical mode and echo
    termios.c_cc[VMIN] = 1;
    termios.c_cc[VTIME] = 0;
    tcsetattr(stdin_fd, TCSANOW, &termios)?;

    Ok(original)
}

/// Disables raw mode for unix terminals
///
/// Note: Not supported on Windows
pub fn disable_raw_mode(original: &Termios) {
    let _ = tcsetattr(tokio::io::stdin().as_raw_fd(), TCSANOW, original);
}

/// Prevents the raw mode to keep being active after the program ends
pub struct RawModeGuard {
    termios: termios::Termios,
}

impl RawModeGuard {
    pub fn new() -> Result<Self, std::io::Error> {
        let termios =
            enable_raw_mode().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(Self { termios })
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        disable_raw_mode(&self.termios);
    }
}
