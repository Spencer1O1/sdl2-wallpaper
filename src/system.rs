use crate::SystemError;
use std::ffi::c_void;

extern "C" {
    fn get_wallpaper_hwnd() -> *mut c_void;
}

pub fn get_hwnd() -> Result<*mut c_void, SystemError> {
    let hwnd = unsafe { get_wallpaper_hwnd() };
    match hwnd.is_null() {
        true => Err(SystemError::HwndAccessFailure),
        false => Ok(hwnd),
    }
}
