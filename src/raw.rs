//! Raw instructions that exclusively print without additional checks.

use std::borrow::Cow;
use std::fmt::Display;
use std::io::Write;

use crate::core::Prefix;

type Result = std::io::Result<()>;

#[derive(Debug)]
pub struct Cargo {
    inner: Prefix,
}

impl Default for Cargo {
    #[inline(always)]
    fn default() -> Self {
        Self { inner: Prefix { prefix: Cow::Borrowed("cargo:"), ..Default::default() } }
    }
}

impl AsRef<Prefix> for Cargo {
    #[inline(always)]
    fn as_ref(&self) -> &Prefix {
        &self.inner
    }
}

/// Write content to [`Out`], automatically handling implementation details.
macro_rules! out {
    ($self:ident, $($arg:tt)*) => {{
        let out = &mut $self.inner.out;
        let lock = out.lock();

        write!(out, "{}", $self.inner.prefix)?;
        writeln!(out, $($arg)*)?;
        drop(lock);
        out.flush()?;

        Ok(())
    }}
}

impl Cargo {
    /// Turn [`Cargo`] into the [`Prefix`] it was wrapping.
    #[inline(always)]
    pub fn into_inner(self) -> Prefix {
        self.inner
    }

    /// Tells Cargo when to re-run the script.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed>
    pub fn rerun_if_changed(&mut self, path: impl Display) -> Result {
        out!(self, "rerun-if-changed={path}")
    }

    /// Tells Cargo when to re-run the script.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed>
    pub fn rerun_if_env_changed(&mut self, var: impl Display) -> Result {
        out!(self, "rerun-if-env-changed={var}")
    }

    /// Passes custom flags to a linker for benchmarks, binaries, cdylib crates, examples, and tests.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg>
    pub fn rustc_link_arg(&mut self, flag: impl Display) -> Result {
        out!(self, "rustc-link-arg={flag}")
    }

    /// Passes custom flags to a linker for the binary `bin`.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bin>
    pub fn rustc_link_arg_bin(&mut self, bin: impl Display, flag: impl Display) -> Result {
        out!(self, "rustc-link-arg-bin={bin}={flag}")
    }

    /// Passes custom flags to a linker for binaries.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bins>
    pub fn rustc_link_arg_bins(&mut self, flag: impl Display) -> Result {
        out!(self, "rustc-link-arg-bins={flag}")
    }

    /// Passes custom flags to a linker for tests.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-tests>
    pub fn rustc_link_arg_tests(&mut self, flag: impl Display) -> Result {
        out!(self, "rustc-link-arg-tests={flag}")
    }

    /// Passes custom flags to a linker for examples.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-examples>
    pub fn rustc_link_arg_examples(&mut self, flag: impl Display) -> Result {
        out!(self, "rustc-link-arg-examples={flag}")
    }

    /// Passes custom flags to a linker for benchmarks.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-benches>
    pub fn rustc_link_arg_benches(&mut self, flag: impl Display) -> Result {
        out!(self, "rustc-link-arg-benches={flag}")
    }

    /// Adds a library to link.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>
    pub fn rustc_link_lib(&mut self, lib: impl Display) -> Result {
        out!(self, "rustc-link-lib={lib}")
    }

    /// Adds to the library search path.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search>
    pub fn rustc_link_search(&mut self, kind: Option<impl Display>, path: impl Display) -> Result {
        match kind {
            Some(kind) => out!(self, "rustc-link-search={kind}={path}"),
            None => out!(self, "rustc-link-search={path}"),
        }
    }

    /// Passes certain flags to the compiler.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-flags>
    pub fn rustc_flags(&mut self, flags: impl Display) -> Result {
        out!(self, "rustc-flags={flags}")
    }

    /// Enables compile-time cfg settings.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg>
    pub fn rustc_cfg(&mut self, key: impl Display, value: Option<impl Display>) -> Result {
        match value {
            Some(value) => out!(self, "rustc-cfg={key}={value}"),
            None => out!(self, "rustc-cfg={key}"),
        }
    }

    /// Sets an environment variable.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env>
    pub fn rustc_env(&mut self, var: impl Display, value: impl Display) -> Result {
        out!(self, "rustc-env={var}={value}")
    }

    /// Passes custom flags to a linker for cdylib crates.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg>
    pub fn rustc_cdylib_link_arg(&mut self, flag: impl Display) -> Result {
        out!(self, "rustc-cdylib-link-arg={flag}")
    }

    /// Displays a warning on the terminal.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning>
    pub fn warning(&mut self, message: impl Display) -> Result {
        out!(self, "warning={message}")
    }

    /// Metadata, used by links scripts.
    ///
    /// <https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key>
    pub fn metadata(&mut self, key: impl Display, value: impl Display) -> Result {
        out!(self, "{key}={value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Out;

    /// Creates a [`Cargo`] that uses an in-memory buffer for output.
    fn cargo_buffer() -> Cargo {
        Cargo { inner: Prefix { prefix: "cargo:".into(), out: Out::Buffer(Vec::new()) } }
    }

    /// Grab the buffer as a [`String`].
    fn buffer_value(cargo: &Cargo) -> String {
        match &cargo.inner.out {
            Out::Buffer(buffer) => String::from_utf8_lossy(buffer).to_string(),

            // we don't need to error, being empty should already be a bad match content-wise
            _ => String::new(),
        }
    }

    #[test]
    fn rerun_if_changed() {
        let mut cargo = cargo_buffer();
        cargo.rerun_if_changed("asdf.txt").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rerun-if-changed=asdf.txt\n");
    }

    #[test]
    fn rerun_if_env_changed() {
        let mut cargo = cargo_buffer();
        cargo.rerun_if_env_changed("ASDF").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rerun-if-env-changed=ASDF\n");
    }

    #[test]
    fn rustc_link_arg() {
        let mut cargo = cargo_buffer();
        cargo.rustc_link_arg("-static").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-arg=-static\n");
    }

    #[test]
    fn rustc_link_arg_bin() {
        let mut cargo = cargo_buffer();
        cargo.rustc_link_arg_bin("cli", "-static").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-arg-bin=cli=-static\n");

        let mut cargo = cargo_buffer();
        cargo.rustc_link_arg_bin("server", "-shared").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-arg-bin=server=-shared\n");
    }

    #[test]
    fn rustc_link_arg_bins() {
        let mut cargo = cargo_buffer();
        cargo.rustc_link_arg_bins("-static").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-arg-bins=-static\n");
    }

    #[test]
    fn rustc_link_arg_tests() {
        let mut cargo = cargo_buffer();
        cargo.rustc_link_arg_tests("-static").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-arg-tests=-static\n");
    }

    #[test]
    fn rustc_link_arg_examples() {
        let mut cargo = cargo_buffer();
        cargo.rustc_link_arg_examples("-static").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-arg-examples=-static\n");
    }

    #[test]
    fn rustc_link_arg_benches() {
        let mut cargo = cargo_buffer();
        cargo.rustc_link_arg_benches("-static").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-arg-benches=-static\n");
    }

    #[test]
    fn rustc_link_lib() {
        let mut cargo = cargo_buffer();
        cargo.rustc_link_lib("static:+whole-archive=mylib").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-lib=static:+whole-archive=mylib\n");
    }

    #[test]
    fn rustc_link_search() {
        let mut cargo = cargo_buffer();
        cargo.rustc_link_search(Option::<&str>::None, "mylib").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-search=mylib\n");

        let mut cargo = cargo_buffer();
        cargo.rustc_link_search(Some("crate"), "mylib").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-link-search=crate=mylib\n");
    }

    #[test]
    fn rustc_flags() {
        let mut cargo = cargo_buffer();
        cargo.rustc_flags("-Clto").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-flags=-Clto\n");
    }

    #[test]
    fn rustc_cfg() {
        let mut cargo = cargo_buffer();
        cargo.rustc_cfg("asdf", Some("hjkl")).unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-cfg=asdf=hjkl\n");

        let mut cargo = cargo_buffer();
        cargo.rustc_cfg("asdf", Option::<&str>::None).unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-cfg=asdf\n");
    }

    #[test]
    fn rustc_env() {
        let mut cargo = cargo_buffer();
        cargo.rustc_env("EDITOR", "vim").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-env=EDITOR=vim\n");
    }

    #[test]
    fn rustc_cdylib_link_arg() {
        let mut cargo = cargo_buffer();
        cargo.rustc_cdylib_link_arg("-pie").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:rustc-cdylib-link-arg=-pie\n");
    }

    #[test]
    fn warning() {
        let mut cargo = cargo_buffer();
        cargo.warning("teapot").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:warning=teapot\n");
    }

    #[test]
    fn metadata() {
        let mut cargo = cargo_buffer();
        cargo.metadata("asdf", "hjkl").unwrap();
        assert_eq!(buffer_value(&cargo), "cargo:asdf=hjkl\n");
    }
}
