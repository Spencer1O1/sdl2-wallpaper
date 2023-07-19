pub use sdl2::IntegerOrSdlError;

#[derive(Debug)]
pub enum SystemError {
    HwndAccessFailure,
}

#[derive(Debug)]
pub enum WindowError {
    SystemError(SystemError),
    SdlError(String),
}

#[derive(Debug)]
pub enum CanvasError {
    WindowError(WindowError),
    IntegerOrSdlError(IntegerOrSdlError)
}
