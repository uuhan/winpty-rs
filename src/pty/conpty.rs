/// This module provides a [`super::PTY`] backend that uses
/// [conpty](https://docs.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session) as its implementation.
/// This backend is available on Windows 10 starting from build number 1809.

// Actual implementation if winpty is available
#[cfg(feature="conpty")]
mod pty_impl;
mod calls;

#[cfg(all(feature="conpty", feature="conpty_local"))]
mod bindings;

// NtCreateNamedPipeFile 的绑定，来自 ntdll，跟用哪个 ConPTY 后端无关。
// 上游把它一并锁在 `conpty_local` 后面，于是系统 ConPTY 那条路引用不到。
#[cfg(feature="conpty")]
mod win_bindings;

#[cfg(feature="conpty")]
pub use pty_impl::ConPTY;

// Default implementation if winpty is not available
#[cfg(not(feature="conpty"))]
mod default_impl;

#[cfg(not(feature="conpty"))]
pub use default_impl::ConPTY;