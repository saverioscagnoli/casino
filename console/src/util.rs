use std::{os::unix::io::AsRawFd, pin::Pin};
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

// pub struct AsyncCallback<R>(
//     Box<dyn Fn(&mut tokio::io::Stdout, String) -> BoxFuture<'static, R> + Send + 'static>,
// );

// impl<R> AsyncCallback<R> {
//     /// Creates a new `AsyncCallback` from a function.
//     ///
//     /// # Arguments
//     ///
//     /// * `f` - A function that takes a mutable reference to `tokio::io::Stdout` and a `&str`,
//     ///         and returns a future.
//     pub fn new<F>(f: F) -> AsyncCallback<R>
//     where
//         F: Fn(&mut tokio::io::Stdout, String) -> BoxFuture<'static, R> + Send + 'static,
//     {
//         Self(Box::new(f))
//     }

//     /// Calls the wrapped asynchronous callback.
//     ///
//     /// # Arguments
//     ///
//     /// * `stdout` - A mutable reference to `tokio::io::Stdout`.
//     /// * `input` - A string slice to pass to the callback.
//     ///
//     /// # Returns
//     ///
//     /// A `BoxFuture` representing the asynchronous operation.
//     pub fn call(&self, stdout: &mut tokio::io::Stdout, input: String) -> BoxFuture<'static, R> {
//         (self.0)(stdout, input)
//     }
// }

// Rust magic to allow storing an an async callback
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type BoxAsyncFn = Box<
    dyn Fn(tokio::io::Stdout, String) -> BoxFuture<'static, tokio::io::Result<()>> + Send + Sync,
>;
pub fn box_async_fn<F, R>(f: F) -> BoxAsyncFn
where
    F: Fn(tokio::io::Stdout, String) -> R + Send + Sync + 'static,
    R: Future<Output = tokio::io::Result<()>> + Send + 'static,
{
    Box::new(move |s, b| Box::pin(f(s, b)))
}
