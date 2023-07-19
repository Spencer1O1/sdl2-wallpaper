pub mod error;
mod system;

use crate::error::{CanvasError, SystemError, WindowError};
use crate::system::get_hwnd;
use sdl2::render::Canvas;
use sdl2::sys::SDL_CreateWindowFrom;
use sdl2::video::Window;

pub fn get_window(sdl_video_subsystem: sdl2::VideoSubsystem) -> Result<Window, WindowError> {
    let hwnd = match get_hwnd() {
        Ok(h) => h,
        Err(system_error) => return Err(WindowError::SystemError(system_error)),
    };
    let raw = unsafe { SDL_CreateWindowFrom(hwnd) };
    if raw.is_null() {
        Err(WindowError::SdlError(sdl2::get_error()))
    } else {
        Ok(unsafe { Window::from_ll(sdl_video_subsystem, raw) })
    }
}

pub fn get_canvas(
    sdl_video_subsystem: sdl2::VideoSubsystem,
) -> Result<Canvas<Window>, CanvasError> {
    let window = match get_window(sdl_video_subsystem) {
        Ok(window) => window,
        Err(window_error) => return Err(CanvasError::WindowError(window_error)),
    };
    match window.into_canvas().build() {
        Ok(canvas) => Ok(canvas),
        Err(int_or_sdl_err) => Err(CanvasError::IntegerOrSdlError(int_or_sdl_err)),
    }
}
