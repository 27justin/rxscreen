//!
//! Exposes the X11 `MIT-SHM` extension.
//!
//! This module contains the interface to the MIT-SHM x11 extension
//! which allows for zero-copy sharing of data between the X server.
//! This is useful for applications which have to capture screenshots in rapid succession,
//! as for example, recording software.
//!
//! [MIT-SHM](https://en.wikipedia.org/wiki/MIT-SHM)
//! [X11 Extension](https://www.x.org/releases/X11R7.7/doc/xextproto/shm.html)
//!
//! # Usage
//! ```rust
//! # use rxscreen::display;
//! #![feature = "xrandr"]
//! if let Ok(display) = Display::new(":0.0") {
//!     let shm = display.shm();
//!     // Now we can configure the module to run at specific coordinates, and specific sizes.
//!     // In this example we will only capture the primary monitor, take a look at the
//!     // `ShmBuilder` functions for more configuration examples.
//!     if let Some(shm) = shm.monitor(display.monitors().iter().find(|m| m.primary()).unwrap())
//!             .build() {
//!         // Now the Shm connection is estabilished and we can take screencaptures through
//!         // `SharedSession::capture` which returns the same struct that `Display::capture` does.
//!     }
//! }
//!
//! ```
//!
//!


use crate::{Display, Image, ffi::{*, constants::*}};
use std::ops::Deref;
use std::pin::Pin;

#[derive(PartialEq, Debug)]
pub enum ShmError {
    ExtensionNotAvailable,
    ShmInitFailed,
    ShmAttachFailed
}


/// Struct dependent on `Display` for Shm
pub struct SharedSession<'a> {
    display: &'a Display,
    shminfo: Pin<Box<XShmSegmentInfo>>,
    image: Image,
    offset: (u32, u32),
    area: (u32, u32),
}
impl<'a> SharedSession<'a> {
    pub fn capture(&self) -> Result<&Image, ()> {
        unsafe {
            if XShmGetImage(self.display.connection, self.display.window, self.image.raw, self.offset.0 as i32, self.offset.1 as i32, AllPlanes) == 1 {
                Ok(&self.image)
            }else{
                Err(())
            }
        }
    }
    pub fn offset(&self) -> (u32, u32) {
        self.offset
    }
    pub fn area(&self) -> (u32, u32) {
        self.area
    }
}
impl<'a> Drop for SharedSession<'a> {
    fn drop(&mut self) {
        use libc::{shmdt, shmctl, shmid_ds, IPC_RMID, c_void};
        unsafe {
            XShmDetach(self.display.connection, self.shminfo.as_ref().get_ref());
            shmdt((*self.shminfo).shmaddr as *const c_void);
            shmctl((*self.shminfo).shmid, IPC_RMID, 0 as *mut shmid_ds);
        }
    }
}

pub struct ShmBuilder<'a> {
    display: &'a Display,
    offset: (u32, u32),
    area: (u32, u32)
}

impl<'a> ShmBuilder<'a> {
    #[cfg(feature = "xrandr")]
    /// Configure the SHM Session to capture a specific monitor.
    /// ```rust
    /// # use rxscreen::{Display, Monitor, ShmBuilder};
    /// if let Ok(display) = Display::new(":0.0") {
    ///     if let Some(shm) = display.shm()
    ///                 .monitor(
    ///                     display.monitors()
    ///                             .iter()
    ///                             .find(|monitor| monitor.primary())
    ///                             .unwrap()
    ///                 ).build() {
    ///         // Do something with the shm session
    ///      }
    /// }
    /// ```
    pub fn monitor(mut self, monitor: &crate::monitor::Monitor) -> Self {
        self.offset = (monitor.x as u32, monitor.y as u32);
        self.area = (monitor.width as u32, monitor.height as u32);
        self
    }
    /// Configure the shm module to take captures of every monitor
    ///
    pub fn full(mut self) -> Self {
        self.offset = (0, 0);
        self.area = (self.display.width as u32, self.display.height as u32);
        self
    }
    /// Set the offset (x, y) and the area (width, height) at which the shared session
    /// should take captures at.
    pub fn area(mut self, offset: (u32, u32), area: (u32, u32)) -> Self {
        self.offset = offset;
        self.area = area;
        self
    }
    /// Initialize the session with the configured parameters.
    /// Returns `None` if the initialization failed.
    /// ```rust
    /// # use rxscreen::{Display, ShmBuilder};
    /// if let Ok(display) = Display::new(":0.0") {
    ///    if let Some(shm) = display.shm()
    ///             .full()
    ///             .build() {
    ///             let capture = shm.capture()
    ///                             .unwrap();
    ///             // Do something with the shm session
    ///         }else{
    ///             // Initialization failed
    ///     }
    /// }
    pub fn build(self) -> Result<SharedSession<'a>, ShmError> {
        use shm::*;
        unsafe {
            use libc::{shmget, shmat};
            use libc::{IPC_PRIVATE, IPC_CREAT};
            use libc::c_void;

            if XShmQueryExtension(self.display.connection) {
                let vis = XDefaultVisual(self.display.connection, 0);

                //let mut shminfo = libc::malloc(std::mem::size_of::<XShmSegmentInfo>()) as *mut XShmSegmentInfo;
                //libc::memset(shminfo as *mut c_void, 0, std::mem::size_of::<XShmSegmentInfo>());
                let mut shminfo = Box::pin(XShmSegmentInfo { shmseg: 0, shmid: 0, shmaddr: std::ptr::null(), read_only: 0});

                let mut ximg = XShmCreateImage(self.display.connection, vis, (*vis).bits_per_rgb as u32, ZPixmap as i32, std::ptr::null(), shminfo.as_ref().get_ref(), self.area.0, self.area.1) as *mut XImage;

                shminfo.shmid = shmget(IPC_PRIVATE, ((*ximg).bytes_per_line * (*ximg).height) as usize, IPC_CREAT|0o600);
                if shminfo.shmid == -1 {
                    return Err(ShmError::ShmInitFailed);
                }

                let memory_addr = shmat((*shminfo).shmid, 0 as *const c_void, 0) as *mut i8;
                shminfo.shmaddr = memory_addr;
                (*ximg).data = memory_addr;
                shminfo.read_only = 0;

                if XShmAttach(self.display.connection, shminfo.as_ref().get_ref()) == 1 {
                    let shared = SharedSession {
                        display: self.display,
                        shminfo,
                        image: Image {
                            raw: ximg
                        },
                        offset: self.offset,
                        area: self.area
                    };
                    Ok(shared)
                }else{
                    Err(ShmError::ShmAttachFailed)
                }
            }else{
                Err(ShmError::ExtensionNotAvailable)
            }
        }
    }
}


impl crate::Display {
    pub fn shm(&self) -> ShmBuilder {
        ShmBuilder {
            area: (0, 0),
            offset: (0, 0),
            display: self
        }
    }
}

#[test]
fn can_create_shm_sessions() {
    let display = Display::new(":0").unwrap();
    let shm = display.shm()
        .area((0, 0), (100, 100))
        .build()
        .unwrap();
    assert!(shm.capture().is_ok());
}
