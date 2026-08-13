//! 刻意什么都不做。
//!
//! 上游的 build.rs 会在 Win10 build >= 2004 上强制去找独立的 ConPTY 二进制，
//! 找不到就调 NuGet 装 `Microsoft.Windows.Console.ConPTY`，然后把
//! `conpty.dll` 链进来（并设 `conpty_local`）。那条路对使用方是有代价的：
//! 产出的 exe 依赖 `conpty.dll` + `OpenConsole.exe`，而这些文件躺在 cargo
//! registry 的源码目录里，`cargo install` 不会带走，清一次缓存就没了。
//! 它还会在 PATH 上发现 `winpty-agent` 时自动把 winpty 后端也链进去 ——
//! 依赖集取决于构建机上装了什么，换台机器结果就不一样。
//!
//! 不设 `conpty_local` 时，`conpty/calls.rs` 走的是 `windows` crate 里的
//! `CreatePseudoConsole` / `ResizePseudoConsole` / `ClosePseudoConsole`，
//! 也就是**系统自带的** ConPTY（Win10 1809+ 随系统提供）。零额外文件，
//! 正是我们要的。
fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    // winpty 后端得显式开 feature 才链，不按构建机上有什么来猜。
    #[cfg(feature = "winpty")]
    println!("cargo:rustc-link-lib=static=winpty");
}
