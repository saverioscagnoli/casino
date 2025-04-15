use async_trait::async_trait;
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// Abstraction over an operation that can be performed
/// on an ANSI output, such as a terminal.
///
/// This trait is used to define operations that can be represented
/// as ANSI escape sequences. These operations can include tasks
/// like clearing the screen, moving the cursor, changing text color,
/// or other terminal manipulations.
///
/// # Examples
///
/// Implementing the `Op` trait for a custom operation:
///
/// ```no_run
/// struct ClearScreen;
///
/// impl Op for ClearScreen {
///     fn ansi(&self) -> String {
///         "\x1B[2J\x1B[H".to_string() // ANSI escape sequence to clear the screen
///     }
/// }
///
/// let clear = ClearScreen;
/// assert_eq!(clear.ansi(), "\x1B[2J\x1B[H");
/// ```
///
/// This trait is typically used in conjunction with the `CommandExecutor`
/// trait to execute operations on an ANSI-compatible output.
pub trait Op {
    /// Returns the string representation of the operation as an ANSI escape sequence.
    ///
    /// This string can be written to an ANSI-compatible output (e.g., a terminal)
    /// to perform the operation. For example, clearing the screen or moving the cursor.
    ///
    /// # Returns
    ///
    /// A `String` containing the ANSI escape sequence for the operation.
    fn ansi(&self) -> String;
}

/// A trait for executing operations that implement the `Op` trait.
///
/// This trait is typically implemented for types that represent
/// writable outputs, such as `tokio::io::Stdout`. It allows
/// asynchronous execution of operations by writing their ANSI
/// escape sequences to the output.
///
/// # Examples
///
/// ```no_run
/// use tokio::io::stdout;
/// use async_trait::async_trait;
///
/// struct ClearScreen;
///
/// impl Op for ClearScreen {
///     fn ansi(&self) -> String {
///         "\x1B[2J\x1B[H".to_string()
///     }
/// }
///
/// #[tokio::main]
/// async fn main() -> tokio::io::Result<()> {
///     let mut stdout = stdout();
///     stdout.execute(ClearScreen).await?;
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait CommandExecutor {
    /// Executes an operation by writing its ANSI escape sequence
    /// to the output.
    ///
    /// # Arguments
    ///
    /// * `command` - The operation to execute, which must implement the `Op` trait.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    async fn execute<O: Op + Send + Sync>(&mut self, op: O) -> tokio::io::Result<()>;
}

/// Implementation of the `CommandExecutor` trait for writable outputs.
///
/// This implementation allows any type that implements `AsyncWrite`
/// and `AsyncWriteExt` to execute operations that implement the `Op` trait.
#[async_trait]
impl<W: AsyncWrite + AsyncWriteExt + Unpin + Send> CommandExecutor for W {
    async fn execute<O: Op + Send>(&mut self, op: O) -> tokio::io::Result<()> {
        let ansi = op.ansi();

        self.write_all(ansi.as_bytes()).await?;
        self.flush().await?;

        Ok(())
    }
}

/// A trait for defining line-based commands in a console application.
///
/// This trait is used to define commands that are triggered by specific
/// input lines in a terminal. Each command has a name, a description,
/// and an asynchronous execution method.
///
/// # Examples
///
/// ```no_run
/// use async_trait::async_trait;
/// use tokio::io::Stdout;
///
/// struct ClearCommand;
///
/// #[async_trait]
/// impl LineCommand for ClearCommand {
///     fn name(&self) -> &str {
///         "clear"
///     }
///
///     fn description(&self) -> &str {
///         "Clears the console screen."
///     }
///
///     async fn execute(
///         &mut self,
///         stdout: &mut Stdout,
///         _args: Vec<&str>,
///     ) -> tokio::io::Result<()> {
///         stdout.write_all(b"\x1B[2J\x1B[H").await?;
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Command: Send + Sync {
    /// Returns the name of the command.
    ///
    /// This name is used to identify the command when parsing input lines.
    fn name(&self) -> &str;

    /// Returns the description of the command.
    ///
    /// This description can be used for help messages or documentation.
    fn description(&self) -> &str;

    /// Executes the command asynchronously.
    ///
    /// # Arguments
    ///
    /// * `stdout` - A mutable reference to the standard output.
    /// * `args` - A vector of arguments passed to the command.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the command execution.
    async fn execute(
        &mut self,
        stdout: &mut tokio::io::Stdout,
        args: Vec<&str>,
    ) -> tokio::io::Result<()>;
}
