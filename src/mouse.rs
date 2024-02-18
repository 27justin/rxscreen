use crate::Display;
use core::ffi::{c_int, c_uint};

impl Display {
    #[cfg(feature = "xrandr")]
    /// Query the mouse position relative to the root window
    /// ```rust
    /// # use rxscreen::Display;
    /// if let Ok(display) = Display::new(":0.0") {
    ///    let Some((mouse_x, mouse_y)) = display.root_mouse_position() else {
    ///         println!("Failed to get mouse position");
    ///    }
    ///    println!("Mouse Pos: {}, {}", mouse_x, mouse_y);
    /// }
    pub fn root_mouse_position(&self) -> Option<(i32, i32)> {
        use crate::ffi::{mouse::XQueryPointer, XWindow};
        unsafe {
            let mut root: XWindow = 0;
            let mut child: XWindow = 0;

            let mut root_x: c_int = 0;
            let mut root_y: c_int = 0;

            let mut win_x: c_int = 0;
            let mut win_y: c_int = 0;

            let mut mask: c_uint = 0;

            let code = XQueryPointer(
                self.connection,
                self.window,
                &mut root,
                &mut child,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask,
            );

            if code != 0 {
                Some((root_x, root_y))
            } else {
                None
            }
        }
    }
}
