use std::{ffi::c_void, os::raw::{c_char, c_int, c_ulong}};


pub const XYBitmap: c_int = 0;
pub const XYPixmap: c_int = 1;
pub const ZPixmap: c_int = 2;
pub const AllPlanes: c_ulong = !0;


#[repr(C)]
pub struct XImage {
    pub width: c_int,
    pub height: c_int,
    pub xoffset: c_int,
    pub format: c_int,
    pub data: *mut c_char,
    pub byte_order: c_int,
    pub bitmap_unity: c_int,
    pub bitmap_bit_order: c_int,
    pub bitmap_pad: c_int,
    pub depth: c_int,
    pub bytes_per_line: c_int,
    pub bits_per_pixel: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub obdata: *mut c_void
}

#[repr(C)]
pub struct Bgr8 {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub pad: u8
}

#[repr(C)]
pub struct Rgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}