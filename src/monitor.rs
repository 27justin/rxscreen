use crate::{Display, ffi::{xrandr::{*}, XGetAtomName}};

impl Display {
    #[cfg(feature = "xrandr")]
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
}
