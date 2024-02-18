use core::ffi::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_void};
use super::{XDisplay, XWindow, Time, Drawable, Atom, Colormap, XID};

pub const KeyPress: u32 = 2;
pub const KeyRelease: u32 = 3;
pub const ButtonPress: u32 = 4;
pub const ButtonRelease: u32 = 5;
pub const MotionNotify: u32 = 6;
pub const EnterNotify: u32 = 7;
pub const LeaveNotify: u32 = 8;
pub const FocusIn: u32 = 9;
pub const FocusOut: u32 = 10;
pub const KeymapNotify: u32 = 11;
pub const Expose: u32 = 12;
pub const GraphicsExpose: u32 = 13;
pub const NoExpose: u32 = 14;
pub const VisibilityNotify: u32 = 15;
pub const CreateNotify: u32 = 16;
pub const DestroyNotify: u32 = 17;
pub const UnmapNotify: u32 = 18;
pub const MapNotify: u32 = 19;
pub const MapRequest: u32 = 20;
pub const ReparentNotify: u32 = 21;
pub const ConfigureNotify: u32 = 22;
pub const ConfigureRequest: u32 = 23;
pub const GravityNotify: u32 = 24;
pub const ResizeRequest: u32 = 25;
pub const CirculateNotify: u32 = 26;
pub const CirculateRequest: u32 = 27;
pub const PropertyNotify: u32 = 28;
pub const SelectionClear: u32 = 29;
pub const SelectionRequest: u32 = 30;
pub const SelectionNotify: u32 = 31;
pub const ColormapNotify: u32 = 32;
pub const ClientMessage: u32 = 33;
pub const MappingNotify: u32 = 34;
pub const GenericEvent: u32 = 35;
pub const LASTEvent: u32 = 36;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XKeyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub root: XWindow,
    pub subwindow: XWindow,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub keycode: c_uint,
    pub same_screen: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XButtonEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub root: XWindow,
    pub subwindow: XWindow,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub button: c_uint,
    pub same_screen: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMotionEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub root: XWindow,
    pub subwindow: XWindow,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub is_hint: c_char,
    pub same_screen: c_int,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCrossingEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub root: XWindow,
    pub subwindow: XWindow,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub mode: c_int,
    pub detail: c_int,
    pub same_screen: c_int,
    pub focus: c_int,
    pub state: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XFocusChangeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub mode: c_int,
    pub detail: c_int,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XKeymapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub key_vector: [c_char; 32usize],
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XExposeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGraphicsExposeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub drawable: Drawable,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int,
    pub major_code: c_int,
    pub minor_code: c_int,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XNoExposeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub drawable: Drawable,
    pub major_code: c_int,
    pub minor_code: c_int,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XVisibilityEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub state: c_int,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCreateWindowEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub parent: XWindow,
    pub window: XWindow,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub override_redirect: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XDestroyWindowEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XUnmapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub from_configure: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub override_redirect: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMapRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub parent: XWindow,
    pub window: XWindow,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XReparentEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub parent: XWindow,
    pub x: c_int,
    pub y: c_int,
    pub override_redirect: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XConfigureEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: XWindow,
    pub override_redirect: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGravityEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XResizeRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub width: c_int,
    pub height: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XConfigureRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub parent: XWindow,
    pub window: XWindow,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: XWindow,
    pub detail: c_int,
    pub value_mask: c_ulong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCirculateEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub place: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCirculateRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub parent: XWindow,
    pub window: XWindow,
    pub place: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPropertyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub atom: Atom,
    pub time: Time,
    pub state: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSelectionClearEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub selection: Atom,
    pub time: Time,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSelectionRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub owner: XWindow,
    pub requestor: XWindow,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSelectionEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub requestor: XWindow,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XColormapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub colormap: Colormap,
    pub new: c_int,
    pub state: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union XClientMessageEvent__bindgen_ty_1 {
    pub b: [c_char; 20usize],
    pub s: [c_short; 10usize],
    pub l: [c_long; 5usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XClientMessageEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub message_type: Atom,
    pub format: c_int,
    pub data: XClientMessageEvent__bindgen_ty_1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMappingEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub request: c_int,
    pub first_keycode: c_int,
    pub count: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XErrorEvent {
    pub type_: c_int,
    pub display: *mut XDisplay,
    pub resourceid: XID,
    pub serial: c_ulong,
    pub error_code: c_uchar,
    pub request_code: c_uchar,
    pub minor_code: c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XAnyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub window: XWindow,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGenericEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub extension: c_int,
    pub evtype: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGenericEventCookie {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut XDisplay,
    pub extension: c_int,
    pub evtype: c_int,
    pub cookie: c_uint,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union XEvent {
    pub r#type: c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [c_long; 24usize],
}
