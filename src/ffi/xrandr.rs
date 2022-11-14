use libc::c_int;

use crate::ffi::*;


extern "C" {
    pub(crate) fn XRRGetMonitors(display: XDisplay, window: XWindow, get_active: c_int /*Bool*/, nmonitors: *mut c_int) -> *const XRRMonitorInfo;
    pub(crate) fn XRRFreeMonitors(monitors: *const XRRMonitorInfo);
}


pub type RROutput = XID;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub(crate) struct XRRMonitorInfo {
    pub(crate) name: Atom,
    pub(crate) primary: c_int, //Bool
    pub(crate) automatic: c_int, //Bool
    pub(crate) noutput: c_int,
    pub(crate) x: c_int,
    pub(crate) y: c_int,
    pub(crate) width: c_int,
    pub(crate) height: c_int,
    pub(crate) mwidth: c_int,
    pub(crate) mheight: c_int,
    pub(crate) outputs: *const RROutput
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


