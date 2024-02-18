//! 
//! Exposes the X11 `xrandr` extension.
//!
//! This module contains the interface to the xrandr x11 extension
//! which allows for querying monitors connected to the display.
//!
//! [xrandr](https://en.wikipedia.org/wiki/Xrandr)
//! ```rust
//! # use rxscreen::Display;
//! if let Ok(display) = Display::new(":0.0") {
//!    let monitors = display.monitors();
//!    for monitor in monitors {
//!        println!("Monitor: {}", monitor.name());
//!        println!("\tPrimary: {}", monitor.primary());
//!        println!("\tSize: {}x{}", monitor.width, monitor.height);
//!        println!("\tPosition: {}x{}", monitor.x, monitor.y);
//!    }
//! }
//!


use crate::{Display, ffi::{xrandr::{*}, XGetAtomName}};

impl Display {
    #[cfg(feature = "xrandr")]
    /// Query every monitor connected to the display
    /// ```rust
    /// # use rxscreen::Display;
    /// if let Ok(display) = Display::new(":0.0") {
    ///    let monitors = display.monitors();
    ///    for monitor in monitors {
    ///        println!("Monitor: {}", monitor.name());
    ///        println!("\tPrimary: {}", monitor.primary());
    ///        println!("\tSize: {}x{}", monitor.width, monitor.height);
    ///        println!("\tPosition: {}x{}", monitor.x, monitor.y);
    ///    }
    /// }
    pub fn monitors(&self) -> Vec<Monitor> {
        use std::ffi::CStr;
        unsafe {
            let mut monitor_count = 0;
            let mut monitors = vec![];
            let ptr = XRRGetMonitors(self.connection, self.window, 0, &mut monitor_count);

            for i in 0..monitor_count {
                let information = &*ptr.offset(i as isize);
                let mon = Monitor {
                    name: CStr::from_ptr(XGetAtomName(self.connection, information.name)).to_str().unwrap().to_owned(),
                    x: information.x,
                    y: information.y,
                    width: information.width,
                    height: information.height,
                    primary: information.primary == 1
                };
                monitors.push(mon);
            }
            XRRFreeMonitors(ptr);
            monitors
        }
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct Monitor {
    pub(crate) name: String,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) primary: bool
}
impl Monitor {
    pub fn name(&self) -> &str {
        &self.name[..]
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn primary(&self) -> bool {
        self.primary
    }

    #[cfg(feature = "mouse")]
    /// Translates a position from Display::root_mouse_position() to a position relative to the monitor
    pub fn mouse_to_local(&self, root_pos: (i32, i32)) -> Option<(i32, i32)> {
        let local_pos = (root_pos.0 - self.x, root_pos.1 - self.y);
        if local_pos.0 >= 0 && local_pos.0 <= self.width && local_pos.1 >= 0 && local_pos.1 <= self.height {
            Some(local_pos)
        } else {
            None
        }
    }
}
