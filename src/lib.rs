//! `rxscreen` is designed to be a simple, easy to use library around various functionality of the
//! x11 server and corresponding extensions.
//! Implemented features:
//! - X11
//! - Xrandr (via the `xrandr` feature)
//! - MIT-SHM (via the `shm` feature)
//!
//! # Examples
//!
//! ### Capture a screenshot of the entire X11 display
//! ```rust
//! # use rxscreen::Display;
//! if let Ok(display) = Display::new(":0.0") {
//!    let capture = display.capture()
//!                     .unwrap();
//!    // `capture` is now an `Image` containing Bgr8 data of the image.
//!    // You can now save it to a file, or do whatever you want with it.
//! }
//! ```
//!
//! ### Capture a screenshot of a specific monitor (requires the "xrandr" feature)
//! ```rust
//! #[cfg(feature = "xrandr")]
//! # use rxscreen::{Display, Monitor};
//! #[cfg(feature = "xrandr")]
//! if let Ok(display) = Display::new(":0.0") {
//!   if let Some(monitor) = display.monitors()
//!                              .iter()
//!                              .find(|monitor| monitor.primary()) {
//!       let capture = display.capture_area(
//!               (monitor.x as u32, monitor.y as u32),
//!               (monitor.width as u32, monitor.height as u32)
//!           ).unwrap();
//!       // `capture` is now an `Image` containing the screenshot of the primary monitor as Bgr8 data.
//!       // You can now save it to a file, or do whatever you want with it.
//!    }
//! }
//!








use std::ffi::CString;

pub mod ffi;
pub use ffi::{Rgb8, Bgr8};
//mod graphics;

#[cfg(feature = "xrandr")]
pub mod monitor;

#[cfg(feature = "shm")]
pub mod shm;

use ffi::{*, constants::{AllPlanes, ZPixmap}};


pub struct Display {
	connection: ffi::XDisplay,
	window: ffi::XWindow,
	pub width: u32,
	pub height: u32,
}

pub struct Image {
	raw: *const XImage
}

#[derive(Debug)]
pub struct DisplayCreationError { description: String }

impl std::fmt::Display for DisplayCreationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl Display {
	/// Open a display to X server using XOpenDisplay at specified display domain
	/// ```rust
	/// # use rxscreen::Display;
	/// if let Ok(display) = Display::new(":0.0") {
	///     // do something with display
	/// }
	/// ```
	/// # Errors
	/// 
	/// If the call to `XOpenDisplay` fails, or if `display_identifier` couldn't be converted to a C String, then this function will
	/// return a DisplayCreationError with details 
	pub fn new(display_identifier: impl Into<String>) -> Result<Self, DisplayCreationError> {
		match CString::new(display_identifier.into()) {
			Ok(location) => {
				let display = unsafe { XOpenDisplay(location.as_ptr()) };
				if !display.is_null() {
					let mut default_window = unsafe { XDefaultRootWindow(display) };
					//let mut default_window = unsafe { XRootWindow(display, 0) };
	
					let (mut width, mut height)= (0u32, 0u32);
					unsafe { XGetGeometry(display, default_window, &mut default_window, &mut 0, 
						&mut 0, &mut width, &mut height, &mut 0, &mut 0) };
	
					Ok(Self {
						connection: display,
						window: default_window,
						width,
						height
					})
				}else{
					Err(DisplayCreationError { description: "Couldn't open display: XOpenDisplay returned NULL".into() })
				}
			},
			Err(x) => Err(DisplayCreationError { description: x.to_string() })
		}
	}

    
	/// Take a screenshot of the display.
	///
	/// ```rust
	/// # use rxscreen::Display;
	/// if let Ok(display) = Display::new(":0.0") {
	///     let capture = display.capture();
	///     #[cfg(feature = "save")]
	///     // With "save" feature enabled
	///     capture.unwrap().save_as("./capture.png");
	///     #[cfg(not(feature = "save"))]
	///     // Access to raw image data without "save" feature
	///     let raw_data = unsafe { capture.unwrap().as_raw_slice() };
	/// }
	/// ```
	///
	/// # Errors
	/// 
	/// This function fails silently if the call to `XGetImage` fails for some reason.
	pub fn capture(&self) -> Result<Image, ()> {
		let image = unsafe { XGetImage(self.connection, self.window, 0, 0, self.width, self.height, AllPlanes, ZPixmap as i32) };
		if !image.is_null() {
			Ok(Image {
			    raw: image
			})
		}else{
			Err(())
		}
	}

	/// Take a screenshot of the provided area.
	///
	/// ```rust
	/// # use rxscreen::Display;
	/// if let Ok(display) = Display::new(":0.0") {
	///     // Capture a 300x300 area of the display, starting at `1920 / 2 - 300` (x)
	///     // and `1080 / 2 - 300` (y)
	///     // i.e. the 300px in the center of the display (if it's a 1920x1080 display)
	///     let capture = display.capture_area((1920 / 2 - 300, 1080 / 2 - 300), (300, 300));
	/// }
	/// ```
	///
	/// # Errors
	/// 
	/// This function fails silently if the call to `XGetImage` fails for some reason.
	pub fn capture_area(&self, offset: (u32, u32), size: (u32, u32)) -> Result<Image, ()> {
		let image = unsafe { XGetImage(self.connection, self.window, offset.0 as i32, offset.1 as i32, size.0, size.1, AllPlanes, ZPixmap as i32) };
		if !image.is_null() {
			Ok(Image {
			    raw: image
			})
		}else{
			Err(())
		}
	}

}

impl Image {
    pub unsafe fn from_raw_parts(display: &Display, data: *const u8, width: u32, height: u32) -> Self {
        let visual = XDefaultVisual(display.connection, 0);
        let ximg = XCreateImage(display.connection, visual, 24, ZPixmap as i32, 0, data as *const i8, width, height, 32, 0);
        // TODO: check ximg for null-ptr
        Self {
            raw: ximg
        }
    }

    pub fn width(&self) -> i32 {
        unsafe {
            (*self.raw).width
        }
    }
    pub fn height(&self) -> i32 {
        unsafe {
            (*self.raw).height
        }
    }

	/// Turns the internal data pointer to a slice of [`Bgr8`]
	///
	/// ```rust
	/// # use rxscreen::{Display, Bgr8, Rgb8};
	/// // Turn Bgr8 into Rgb8
	/// if let Ok(display) = Display::new(":0.0") {
	///     let capture = display.capture();
	///     let rgb_buffer: Vec<Rgb8> = unsafe{ capture.unwrap().as_raw_slice() }
	///                                 .into_iter()
	///                                 .map(|bgr| Rgb8::from(bgr))
	///                                 .collect();
	/// }
	/// ```
	pub unsafe fn as_raw_slice<'a>(&self) -> &'a [Bgr8] {
		let blob_length = ((*self.raw).width * (*self.raw).height) as usize;
		std::slice::from_raw_parts((*self.raw).data as *const Bgr8, blob_length)
	}

    /// Returns a slice of the raw image data
    pub unsafe fn as_bytes<'a>(&'a self) -> &'a [u8] {
        let length = self.width() * self.height() * ((*self.raw).depth / 8);
        std::slice::from_raw_parts((*self.raw).data as *const u8, length as usize)
    }
    pub unsafe fn as_bytes_mut<'a>(&'a mut self) -> &'a mut [u8] {
        let length = self.width() * self.height() * ((*self.raw).depth / 8);
        let slice = std::slice::from_raw_parts_mut((*self.raw).data as *mut u8, length as usize);
        slice
    }

    /// Create a new empty image through the X11 `XCreateImage` function.
    pub fn empty(display: &Display, width: u32, height: u32) -> Self {
        unsafe {
            let visual = XDefaultVisual(display.connection, 0);
            let img_size = (width * height * (32 / 8)) as usize;
            let data = libc::malloc(img_size);

            let ximg = XCreateImage(display.connection, visual, 24, ZPixmap as i32, 0, data as *const i8, width, height, 32, 0);
            Self {
                raw: ximg
            }
        }
    }

    /// Returns the pointer for the internal data buffer.
    pub unsafe fn as_ptr(&self) -> *const u8 {
        (*self.raw).data as *const u8
    }

	#[cfg(feature = "save")]
	/// Saves the image to file
	///
	///	You can save as any filetype that the `image`-crate supports
	///
	/// ```rust
	/// # use rxscreen::Display;
	/// if let Ok(display) = Display::new(":0.0") {
	///		let screenshot = display.capture();
	///		screenshot.unwrap().save_as("./screenshot.png");
	/// }
	/// ```
	///
	/// # Warning
	///
	/// **Without** configuring any opt-level (debug build) this function may take upwards of 10 seconds for one screenshot
	/// (measured on debug build, 5760x1080 capture, ryzen 7 2700x, gtx 1050 ti)
	///
	/// **With opt-level 3** it takes 0.2 seconds for a capture of the same size.
	/// 

	pub fn save_as(&self, file: impl Into<std::path::PathBuf>) -> std::io::Result<()> {
		use image::{save_buffer, ColorType};
		// Restructure buffer to fit RGB instead of BGRP
		let (width, height) = unsafe { ((*self.raw).width, (*self.raw).height) };

		fn thrice<T>(first: T, second: T, third: T) -> impl Iterator<Item = T> {
			use std::iter::once;
			once(first).chain(once(second)).chain(once(third))
		}

		let buffer = unsafe { self.as_raw_slice() };
		let buffer = buffer.into_iter().map(|brg| thrice(brg.r, brg.g, brg.b)).flatten().collect::<Vec<u8>>();

		match save_buffer(file.into(), &buffer, width as u32, height as u32, ColorType::Rgb8) {
			Ok(()) => Ok(()),
			Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Couldn't write to file"))
		}
	}

	#[cfg(feature = "save")]
	/// Saves the raw capture to PNG in memory
	///
	/// ```rust
	///	# use rxscreen::{Display, Image};
	/// if let Ok(display) = Display::new(":0.0") {
	///		let screenshot = display.capture().unwrap();
	///		let data = screenshot.save_to_memory();
	///		// `data` now contains the encoded PNG file
	/// }
	/// ```
	pub fn save_to_memory(self) -> std::io::Result<Vec<u8>>{
		use image::{codecs::png::PngEncoder, ColorType};
		
		fn thrice<T>(first: T, second: T, third: T) -> impl Iterator<Item = T> {
			use std::iter::once;
			once(first).chain(once(second)).chain(once(third))
		}

		// Restructure BGR8 into RGB8
		let buffer = unsafe { self.as_raw_slice() };
		let buffer = buffer.into_iter().map(|brg| thrice(brg.r, brg.g, brg.b)).flatten().collect::<Vec<u8>>();

		let mut png_data: Vec<u8> = vec![];

		let encoder = PngEncoder::new(&mut png_data);
		let (width, height) = unsafe { ((*self.raw).width, (*self.raw).height) };
		encoder.encode(&buffer, width as u32, height as u32, ColorType::Rgb8);
		
		Ok(png_data)
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		unsafe{ crate::XDestroyImage(self.raw); }
	}
}

impl Drop for Display {
	fn drop(&mut self) {
		unsafe { XCloseDisplay(self.connection) };
	}
}



mod tests {
	#[test]
	pub fn can_open_x11_display() {
		assert_eq!(crate::Display::new(":0.0").is_ok(), true);
	}
	#[test]
	pub fn fails_on_incorrect_display() {
		assert_eq!(crate::Display::new("test:5.0").is_err(), true);
	}
}



impl From<Bgr8> for Rgb8 {
	fn from(bgr: Bgr8) -> Self {
		Self {
			r: bgr.r,
			g: bgr.g,
			b: bgr.b
		}
	}
}

impl From<&Bgr8> for Rgb8 {
	fn from(bgr:&Bgr8) -> Self {
		Self {
			r: bgr.r,
			g: bgr.g,
			b: bgr.b
		}
	}
}
