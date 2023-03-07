use std::borrow::Cow;
use std::fmt::{Arguments, Display};
use std::io::{IoSlice, Stdout, StdoutLock, Write};

/// How a tool identifies a build instruction and where to output it.
///
/// ```
/// # use build_instructions::core::{Prefix};
/// // create a prefix used for cargo on stdout
/// let prefix = Prefix { prefix: "cargo:".into(), ..Default::default()};
/// assert_eq!(format!("{}instruction=value", prefix.prefix), "cargo:instruction=value");
/// ```
#[derive(Debug, Default)]
pub struct Prefix {
    /// The prefix for all instructions.
    ///
    /// e.g. `cargo:` for [Cargo's instructions]. Note that if the prefix has a delimiter, such
    /// as `:` in `cargo:`, then this value **should include it**.
    ///
    /// [Cargo's instructions]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script
    pub prefix: Cow<'static, str>,

    /// Where this [`Prefix`] will output instructions.
    pub out: Out,
}

impl Prefix {}

/// Where [`Prefix`] outputs instructions.
#[derive(Debug)]
pub enum Out {
    /// Stdout, what you want to use in build scripts (default).
    Stdout(Stdout),

    /// An in-memory buffer, useful for testing the output of your instructions.
    Buffer(Vec<u8>),
}

impl Default for Out {
    #[inline(always)]
    fn default() -> Self {
        Self::Stdout(std::io::stdout())
    }
}

macro_rules! pass {
    ($self:ident, $fn:ident) => {
        match $self {
            Self::Stdout(stdout) => stdout.$fn(),
            Self::Buffer(buffer) => buffer.$fn(),
        }
    };
    ($self:ident, $fn:ident, $($args:ident),*) => {
        match $self {
            Self::Stdout(stdout) => stdout.$fn($($args),*),
            Self::Buffer(buffer) => buffer.$fn($($args),*),
        }
    };
}

impl Write for Out {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        pass!(self, write, buf)
    }

    #[inline(always)]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        pass!(self, write_vectored, bufs)
    }

    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        pass!(self, flush)
    }

    #[inline(always)]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        pass!(self, write_all, buf)
    }

    #[inline(always)]
    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        pass!(self, write_fmt, fmt)
    }
}

pub struct OutLock(Option<StdoutLock<'static>>);

impl Out {
    /// Lock the output, if applicable.
    ///
    /// This currently only flushed [`Out::Stdout`].
    pub fn lock(&self) -> OutLock {
        OutLock(match self {
            Self::Stdout(stdout) => Some(stdout.lock()),
            Self::Buffer(_) => None,
        })
    }

    /// Flush the output, if applicable.
    ///
    /// This currently only flushes [`Out::Stdout`].
    pub fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Stdout(stdout) => stdout.flush(),
            _ => Ok(()),
        }
    }
}

/// Represents a specific instruction for any prefix.
pub struct Instruction<K: Display, V: Display> {
    /// The name of the instruction.
    ///
    /// e.g. `rerun-if-changed` in [`cargo:rerun-if-changed=PATH`]. This value should **NOT**
    /// include an equals (`=`) delimiter at the end.
    ///
    /// [`cargo:rerun-if-changed=PATH`]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed
    pub name: Cow<'static, str>,

    /// Any format of values set for this specific instruction.
    ///
    /// e.g. `PATH`in
    pub value: Value<K, V>,
}

pub enum Value<K: Display, V: Display> {
    Value(V),
    KeyValue(K, V),
}
