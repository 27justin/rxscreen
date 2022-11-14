use std::os::raw::{c_uint, c_int, c_char, c_ulong};

use crate::ffi::*;

extern "C" {
    pub fn XShmQueryExtension(display: XDisplay) -> bool;
    //pub fn XShmCreateImage(display: XDisplay, visual: *const Visual, depth: c_uint, format: c_int, data: *const c_char, shminfo: *const XShmSegmentInfo, width: c_uint, height: c_uint);
    pub fn XShmCreateImage(display: XDisplay, visual: *const Visual, depth: c_uint, format: c_int, data: *const c_char, shminfo: *const XShmSegmentInfo, width: c_uint, height: c_uint) -> *const XImage;
    pub fn XShmAttach(display: XDisplay, shminfo: *const XShmSegmentInfo) -> c_int;
    pub fn XShmGetImage(display: XDisplay, drawable: XWindow, image: *const XImage, x: c_int, y: c_int, plane_mask: c_ulong) -> c_int;
    pub fn XShmDetach(display: XDisplay, shminfo: *const XShmSegmentInfo) -> c_int;
}





pub type ShmSeg = c_ulong;

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct XShmSegmentInfo {
    pub shmseg: ShmSeg,
    pub shmid: c_int,
    pub shmaddr: *const c_char,
    pub read_only: c_int
}


