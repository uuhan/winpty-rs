// #[cfg(all(feature="conpty", feature="conpty_local"))]
// mod bindings;
#![allow(non_snake_case)]

use windows::core::{Error, Result, HRESULT};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Console::{COORD, HPCON};

use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::os::windows::raw;

#[cfg(all(feature = "conpty", not(feature = "conpty_local")))]
pub use windows::Win32::System::Console::{CreatePseudoConsole, ResizePseudoConsole, ClosePseudoConsole};

/// 系统自带的 ConPTY 没有这个 API —— 它是独立 conpty.dll 的扩展，用来隐藏
/// 伪控制台窗口。调用方只是「有就调一下」（返回值直接丢弃），所以这里给个
/// 空实现，让系统 ConPTY 这条路能编译。
#[cfg(all(feature = "conpty", not(feature = "conpty_local")))]
pub unsafe fn ShowHidePseudoConsole(_hPC: HPCON, _show: bool) -> Result<()> {
    Ok(())
}

#[cfg(all(feature = "conpty", feature = "conpty_local"))]
use super::bindings::{
    ConptyClearPseudoConsole, ConptyClosePseudoConsole, ConptyCreatePseudoConsole,
    ConptyResizePseudoConsole, ConptyShowHidePseudoConsole,
};

#[cfg(all(feature = "conpty", feature = "conpty_local"))]
pub unsafe fn CreatePseudoConsole(
    size: COORD,
    hInput: HANDLE,
    hOutput: HANDLE,
    dwFlags: u32,
) -> Result<HPCON> {
    let mut console_handle_uninit = MaybeUninit::<HPCON>::uninit();
    let result_code = ConptyCreatePseudoConsole(
        size,
        hInput.0 as raw::HANDLE,
        hOutput.0 as raw::HANDLE,
        dwFlags,
        console_handle_uninit.as_mut_ptr() as *mut c_void,
    );

    let result = HRESULT::from_nt(result_code);
    if result.is_err() {
        Err(Error::from_hresult(result))
    } else {
        let console_handle = console_handle_uninit.assume_init();
        Ok(console_handle)
    }
}

#[cfg(all(feature = "conpty", feature = "conpty_local"))]
pub unsafe fn ResizePseudoConsole(hPC: HPCON, size: COORD) -> Result<()> {
    let result_code = ConptyResizePseudoConsole(hPC.0 as *mut c_void, size);

    let result = HRESULT::from_nt(result_code);
    if result.is_err() {
        Err(Error::from_hresult(result))
    } else {
        Ok(())
    }
}

#[cfg(all(feature = "conpty", feature = "conpty_local"))]
pub unsafe fn ClearPseudoConsole(hPC: HPCON) -> Result<()> {
    let result_code = ConptyClearPseudoConsole(hPC.0 as *mut c_void);

    let result = HRESULT::from_nt(result_code);
    if result.is_err() {
        Err(Error::from_hresult(result))
    } else {
        Ok(())
    }
}

#[cfg(all(feature = "conpty", feature = "conpty_local"))]
pub unsafe fn ClosePseudoConsole(hPC: HPCON) -> Result<()> {
    let result_code = ConptyClosePseudoConsole(hPC.0 as *mut c_void);

    let result = HRESULT::from_nt(result_code);
    if result.is_err() {
        Err(Error::from_hresult(result))
    } else {
        Ok(())
    }
}

#[cfg(all(feature = "conpty", feature = "conpty_local"))]
pub unsafe fn ShowHidePseudoConsole(hPC: HPCON, show: bool) -> Result<()> {
    let result_code = ConptyShowHidePseudoConsole(hPC.0 as *mut c_void, show);

    let result = HRESULT::from_nt(result_code);
    if result.is_err() {
        Err(Error::from_hresult(result))
    } else {
        Ok(())
    }
}
