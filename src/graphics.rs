use crate::Display;
use crate::Image;
use crate::ffi::*;
use crate::ffi::constants::Expose;
use crate::ffi::constants::ExposureMask;
use crate::ffi::constants::KeyPressMask;
use crate::ffi::constants::ZPixmap;
use crate::ffi::events::XEvent;


pub trait Drawable {
    fn draw(&self, display: &Display, window: XWindow, gc: GC) -> bool;
}


impl<'a, T: AsRef<str>> Drawable for T
    where T: Copy, Vec<u8>: From<T> {
    fn draw(&self, display: &Display, window: XWindow, gc: GC) -> bool {
        use std::ffi::CString;
        unsafe {
            let cstr = CString::new(*self).unwrap();
            XDrawString(display.connection, window, gc, 30, 30, cstr.clone().into_raw(), cstr.as_bytes_with_nul().len() as i32);
        }
        true
    }
}

impl Drawable for &Image {
    fn draw(&self, display: &Display, window: XWindow, gc: GC) -> bool {
        unsafe {
            XPutImage(display.connection, window, gc, self.raw, 0, 0, 0, 0, (*self.raw).width as u32, (*self.raw).height as u32);
        }
        true
    }
}

pub struct Window<'a> {
    display: &'a Display,
    window: XWindow,
    gc: GC,
    visual: *const Visual
}

impl<'a> Window<'a> {
    pub fn event_loop(&self) -> () {
        unsafe {
            let mut xev = XEvent { r#type: 0 };
            loop {
                XNextEvent(self.display.connection, &mut xev);
                println!("Got Event {}", xev.r#type);
            }
        }
    }
    pub fn draw(&self, drawable: impl Drawable) -> bool {
        drawable.draw(self.display, self.window, self.gc);
        true
    }

}


pub struct WindowBuilder<'a> {
    display: &'a Display,
    offset: (i32, i32),
    size: (u32, u32),
    border_width: u32
}

impl<'a> WindowBuilder<'a> {
    pub fn size(mut self, size: (u32, u32)) -> Self {
        self.size = size;
        self
    }
    pub fn offset(mut self, offset: (i32, i32)) -> Self {
        self.offset = offset;
        self
    }
    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = border_width;
        self
    }
    pub fn build(self) -> Result<Window<'a>, ()> {
        unsafe {
            let wd = XBlackPixel(self.display.connection, 0);
            let ww = XWhitePixel(self.display.connection, 0);
            let window = XCreateSimpleWindow(self.display.connection, self.display.window, 0, 0, 600, 400, 0, 0, 0);

            let visual = XDefaultVisual(self.display.connection, 0);
            XSelectInput(self.display.connection, window, (ExposureMask | KeyPressMask) as u64);
            XMapWindow(self.display.connection, window);
            XFlush(self.display.connection);

            let gc = XCreateGC(self.display.connection, window, 0, std::ptr::null());


            // Wait until X-Server sends the Expose event
            let mut xev = XEvent { r#type: 0 };
            loop {
                XNextEvent(self.display.connection, &mut xev);
                if xev.r#type == Expose as i32 {
                    break;
                }
            }

            Ok(Window {
                display: self.display,
                gc,
                window,
                visual
            })
        }
    }
}


impl Display {
    pub fn new_window<'a>(&'a self) -> WindowBuilder<'a> {
        WindowBuilder {
            display: &self,
            offset: (0,0),
            size: (600, 400),
            border_width: 0
        }
    }
}



#[test]
fn can_create_blank_window() {
    let display = Display::new(":0.0").unwrap();
    display.new_window()
            .size((600, 400))
            .build();
}

