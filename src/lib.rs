use std::fmt::Display;
use std::path::Path;

pub mod core;
pub mod raw;

type Result = std::io::Result<()>;

/// Build instructions to use for Cargo, with minor usability enhancements.
#[derive(Debug, Default)]
pub struct Cargo {
    inner: raw::Cargo,
}

impl From<raw::Cargo> for Cargo {
    fn from(inner: raw::Cargo) -> Self {
        Self { inner }
    }
}

impl Cargo {
    /// Turn [`Cargo`] into the underlying [`raw::Cargo`].
    #[inline(always)]
    pub fn into_inner(self) -> raw::Cargo {
        self.inner
    }

    /// Get a mutable reference to the underlying [`raw::Cargo`].
    #[inline(always)]
    pub fn raw_mut(&mut self) -> &mut raw::Cargo {
        &mut self.inner
    }

    /// Tells Cargo when to re-run the script.
    ///
    /// The path will be checked to see if it exists, and how the instruction is emitted depends
    /// on the [`PathBehavior`] set.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed>
    pub fn rerun_if_changed(&mut self, path: impl AsRef<Path>, behavior: PathBehavior) -> Result {
        let path = path.as_ref();
        let display = path.display();
        match (behavior, path.exists()) {
            (PathBehavior::Always, _) => self.inner.rerun_if_changed(display),
            (PathBehavior::OnlyIfExists, true) => self.inner.rerun_if_changed(display),
            (PathBehavior::OnlyIfExists, false) => Ok(()),
            (PathBehavior::MustExist, true) => self.inner.rerun_if_changed(display),
            (PathBehavior::MustExist, false) => Err(std::io::ErrorKind::NotFound.into()),
        }
    }

    /// Tells Cargo when to re-run the script.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed>
    pub fn rerun_if_env_changed(&mut self, var: impl Display) -> Result {
        todo!()
    }

    /// Passes custom flags to a linker for benchmarks, binaries, cdylib crates, examples, and tests.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg>
    pub fn rustc_link_arg(&mut self, flag: impl Display) -> Result {
        todo!()
    }

    /// Passes custom flags to a linker for the binary `bin`.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bin>
    pub fn rustc_link_arg_bin(&mut self, bin: impl Display, flag: impl Display) -> Result {
        todo!()
    }

    /// Passes custom flags to a linker for binaries.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bins>
    pub fn rustc_link_arg_bins(&mut self, flag: impl Display) -> Result {
        todo!()
    }

    /// Passes custom flags to a linker for tests.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-tests>
    pub fn rustc_link_arg_tests(&mut self, flag: impl Display) -> Result {
        todo!()
    }

    /// Passes custom flags to a linker for examples.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-examples>
    pub fn rustc_link_arg_examples(&mut self, flag: impl Display) -> Result {
        todo!()
    }

    /// Passes custom flags to a linker for benchmarks.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-benches>
    pub fn rustc_link_arg_benches(&mut self, flag: impl Display) -> Result {
        todo!()
    }

    /// Adds a library to link.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
    pub fn rustc_link_lib(&mut self, lib: impl Display) -> Result {
        todo!()
    }

    /// Adds to the library search path.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
    pub fn rustc_link_search(&mut self, kind: Option<impl Display>, path: impl Display) -> Result {
        todo!()
    }

    /// Passes certain flags to the compiler.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-flags>
    pub fn rustc_flags(&mut self, flags: impl Display) -> Result {
        todo!()
    }

    /// Enables compile-time cfg settings.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg>
    pub fn rustc_cfg(&mut self, key: impl Display, value: Option<impl Display>) -> Result {
        todo!()
    }

    /// Sets an environment variable.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env>
    pub fn rustc_env(&mut self, var: impl Display, value: impl Display) -> Result {
        todo!()
    }

    /// Passes custom flags to a linker for cdylib crates.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg>
    pub fn rustc_cdylib_link_arg(&mut self, flag: impl Display) -> Result {
        todo!()
    }

    /// Displays a warning on the terminal.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning>
    pub fn warning(&mut self, message: impl Display) -> Result {
        todo!()
    }

    /// Metadata, used by links scripts.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key>
    pub fn metadata(&mut self, key: impl Display, value: impl Display) -> Result {
        todo!()
    }
}

/// How checking a path should behave for an instruction.
pub enum PathBehavior {
    /// The instruction will only emit if the path exists, otherwise ignored.
    OnlyIfExists,

    /// The path must exist or the instruction error.
    MustExist,

    /// The instruction will always be emitted and the path **not** checked.
    Always,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::core::{Out, Prefix};

    /// Creates a [`Cargo`] that uses an in-memory buffer for output.
    fn cargo_buffer() -> Cargo {
        Cargo { inner: raw::Cargo::new(Out::Buffer(Vec::with_capacity(64))) }
    }

    /// Grab the buffer as a [`String`].
    fn buffer_value(cargo: &Cargo) -> String {
        match &cargo.inner.as_ref().out {
            Out::Buffer(buffer) => String::from_utf8_lossy(buffer).to_string(),

            // we don't need to error, being empty should already be a bad match content-wise
            _ => String::new(),
        }
    }

    #[test]
    fn rerun_if_changed() {}
}
