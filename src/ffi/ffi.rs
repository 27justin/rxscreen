use std::{ffi::{c_void, CString, CStr}, os::raw::{c_char, c_int, c_ulong, c_uint, c_long}};

use super::events::XEvent;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XGC {
    _unused: [u8; 0],
}

pub type XID = c_ulong;
pub type XDisplay = *const c_char;
//pub type GC = CARD32;
pub type GC = *mut _XGC;
pub type Atom = c_ulong;
pub type Time = c_ulong;

pub type XWindow = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type GContext = XID;
pub type KeySym = XID;
pub type XPointer = *mut ::std::os::raw::c_char;

#[cfg(target_arch = "x86_64")]
pub type CARD32 = c_uint;
#[cfg(not(target_arch = "x86_64"))]
pub type CARD32 = c_long;



extern "C" {
	pub fn XOpenDisplay(display: *const c_char) -> XDisplay;
	pub fn XDefaultRootWindow(display: XDisplay) -> XWindow;
	pub fn XGetGeometry(
		display: XDisplay,
		screen: XWindow,
		root_return: &mut XWindow,
		x_return: &mut c_int,
		y_return: &mut c_int,
		width_return: &mut c_uint,
		height_return: &mut c_uint,
		border_width_return: &mut c_uint,
		depth_return: &mut c_uint) -> ();
	pub fn XCloseDisplay(display: XDisplay);
	pub fn XGetImage(display: XDisplay, d: XWindow, x: c_int, y: c_int, width: c_uint, height: c_uint, plane_mask: c_ulong, format: c_int) -> *mut XImage;
    pub fn XCreateImage(display: XDisplay, visual: *const Visual, depth: c_uint, format: c_int, offset: c_int, data: *const c_char, width: c_uint, height: c_uint, bitmap_pad: c_int, bytes_per_line: c_int) -> *mut XImage;
    
	pub fn XDestroyImage(image: *const XImage);
    pub fn XDefaultVisual(display: XDisplay, screen_num: c_int) -> *const Visual;
    pub fn XScreenOfDisplay(display: XDisplay, screen_num: c_int) -> *const Screen;
    pub fn XRootWindowOfScreen(screen: *const Screen) -> XWindow;
    pub fn XDefaultDepth(display: XDisplay, screen_num: c_int) -> c_int;
    pub fn XRootWindow(display: XDisplay, screen_num: c_int) -> XWindow;
    pub fn XGetAtomName(display: XDisplay, atom: Atom) -> *const c_char;

    pub fn XPutImage(display: XDisplay, drawable: XWindow, gc: GC, ximage: *const XImage, src_x: c_int, src_y: c_int, dest_x: c_int, dest_y: c_int, width: c_uint, height: c_uint) -> c_int;
    pub fn XCreateGC(display: XDisplay, drawable: XWindow, valuemask: c_ulong, values: *const XGCValues) -> GC;
    pub fn XCreateSimpleWindow(display: XDisplay, window: XWindow, x: c_int, y: c_int, width: c_uint, height: c_uint, border_width: c_uint, border: c_ulong, background: c_ulong) -> XWindow;
    pub fn XCreateWindow(display: XDisplay, window: XWindow, x: c_int, y: c_int, width: c_uint, height: c_uint, border_width: c_uint, depth: c_int, class: c_uint, visual: *const Visual, valuemask: c_ulong, attributes: *const XSetWindowAttributes) -> XWindow;
    pub fn XBlackPixel(display: XDisplay, screen_num: c_int) -> c_ulong;
    pub fn XWhitePixel(display: XDisplay, screen_num: c_int) -> c_ulong;
    pub fn XMapWindow(display: XDisplay, window: XWindow) -> c_int;
    pub fn XFlushGC(display: XDisplay, gc: GC);
    pub fn XFlush(display: XDisplay) -> c_int;
    pub fn XSelectInput(display: XDisplay, window: XWindow, event_mask: c_ulong) -> c_int;
    pub fn XNextEvent(display: XDisplay, event: *mut XEvent) -> c_int;
    pub fn XSync(display: XDisplay, discard: c_int /* bool */) -> c_int;
    pub fn XDrawString(display: XDisplay, drawable: Drawable, gc: GC, x: c_int, y: c_int, string: *const c_char, strlen: c_int) -> c_int;
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XImage {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub xoffset: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
    pub data: *mut ::std::os::raw::c_char,
    pub byte_order: ::std::os::raw::c_int,
    pub bitmap_unit: ::std::os::raw::c_int,
    pub bitmap_bit_order: ::std::os::raw::c_int,
    pub bitmap_pad: ::std::os::raw::c_int,
    pub depth: ::std::os::raw::c_int,
    pub bytes_per_line: ::std::os::raw::c_int,
    pub bits_per_pixel: ::std::os::raw::c_int,
    pub red_mask: ::std::os::raw::c_ulong,
    pub green_mask: ::std::os::raw::c_ulong,
    pub blue_mask: ::std::os::raw::c_ulong,
    pub obdata: XPointer
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Depth {
    depth: c_int,
    nvisuals: c_int,
    visuals: *const Visual
}




#[derive(Debug, PartialEq)]
#[repr(C)]
// TODO:
// somehow the fields are shifting, when testing with XScreenOfDisplay() width and height
// arrive in fields height and mwidth, therefore I presume that either ext_data, display, or root
// are faulty and have to be removed
pub struct Screen {
    ext_data: *const XExtData,
    display: XDisplay,
    root: XWindow,
    width: c_int,
    height: c_int,
    mwidth: c_int,
    mheight: c_int,
    ndepths: c_int,
    depths: *const Depth,
    root_depth: c_int,
    root_visual: *const Visual,
    default_gc: GC,
    cmap: Colormap,
    white_pixel: c_ulong,
    black_pixel: c_ulong,
    max_maps: c_int,
    min_maps: c_int,
    backing_store: c_int,
    save_unders: c_int, //bool
    root_input_mask: c_long
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

pub type VisualID = CARD32;

#[derive(PartialEq,Debug)]
#[repr(C)]
pub struct XExtData {
    number: c_int,
    next: *const XExtData,
    free_private: fn(extension: *const XExtData) -> c_int,
    private_data: *const char
}

#[derive(PartialEq,Debug)]
#[repr(C)]
pub struct Visual {
    pub ext_data: XExtData,
    pub visualid: VisualID,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub bits_per_rgb: c_int,
    pub map_entries: c_int
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGCValues {
    pub function: c_int,
    pub plane_mask: c_ulong,
    pub foreground: c_ulong,
    pub background: c_ulong,
    pub line_width: c_int,
    pub line_style: c_int,
    pub cap_style: c_int,
    pub join_style: c_int,
    pub fill_style: c_int,
    pub fill_rule: c_int,
    pub arc_mode: c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: c_int,
    pub ts_y_origin: c_int,
    pub font: Font,
    pub subwindow_mode: c_int,
    pub graphics_exposures: c_int,
    pub clip_x_origin: c_int,
    pub clip_y_origin: c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: c_int,
    pub dashes: c_char,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: c_ulong,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: c_int,
    pub event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
