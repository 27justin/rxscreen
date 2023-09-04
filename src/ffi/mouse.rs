use std::os::raw::{c_char, c_int, c_uint, c_ulong};

use crate::ffi::*;

extern "C" {
    pub fn XQueryPointer(
        display: XDisplay,
        w: XWindow,
        root_return: *const XWindow,
        child_return: *const XWindow,
        root_x_return: *const c_int,
        root_y_return: *const c_int,
        win_x_return: *const c_int,
        win_y_return: *const c_int,
        mask_return: *const c_uint,
    ) -> c_int;
}
