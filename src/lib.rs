
use std::{ffi::{CString}, os::raw::{c_char, c_int, c_uint, c_ulong}, u32, path::PathBuf};

mod ffi;
use ffi::*;
pub use ffi::{Bgr8, Rgb8};

type XDisplay = *const c_char;
type Window = u32;

extern "C" {
	fn XOpenDisplay(display: *const c_char) -> XDisplay;
	fn XDefaultRootWindow(display: XDisplay) -> Window;
	// Status XGetWindowAttributes(Display *display, Window w, XWindowAttributes *window_attributes_return); 
	fn XGetGeometry(
		display: XDisplay,
		screen: Window,
		root_return: &mut Window,
		x_return: &mut c_int,
		y_return: &mut c_int,
		width_return: &mut c_uint,
		height_return: &mut c_uint,
		border_width_return: &mut c_uint,
		depth_return: &mut c_uint) -> ();
	fn XCloseDisplay(display: XDisplay);
	fn XGetImage(display: XDisplay, d: Window, x: c_int, y: c_int, width: c_uint, height: c_uint, plane_mask: c_ulong, format: c_int) -> *const XImage;
	fn XDestroyImage(image: *const XImage);
}

pub struct Display {
	connection: *const c_char,
	window: u32,
	pub width: u32,
	pub height: u32,
}

pub struct Image {
	_image: *const XImage,
	width: u32,
	height: u32,
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
	///		// do something with display
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
	///		let screenshot = display.screenshot();
	///		#[cfg(feature = "save")]
	///		// With "save" feature enabled
	///		screenshot.unwrap().save_as("./screenshot.png");
	///		#[cfg(not(feature = "save"))]
	/// 	// Access to raw image data without "save" feature
	///		let raw_data = unsafe { screenshot.unwrap().as_raw_slice() };
	/// }
	/// ```
	///
	/// # Errors
	/// 
	/// This function fails silently if the call to `XGetImage` fails for some reason.
	pub fn screenshot(&self) -> Result<Image, ()> {
		let image = unsafe { XGetImage(self.connection, self.window, 0, 0, self.width, self.height, AllPlanes, ZPixmap) };
		if !image.is_null() {
			Ok(Image {
				_image: image,
				width: self.width,
				height: self.height
			})
		}else{
			Err(())
		}
	}
}

impl Image {
	/// Turns the internal data pointer to a slice of [`Bgr8`]
	///
	/// ```rust
	/// # use rxscreen::{Display, Bgr8, Rgb8};
	/// // Turn Bgr8 into Rgb8
	/// if let Ok(display) = Display::new(":0.0") {
	///		let screenshot = display.screenshot();
	///		let rgb_buffer: Vec<Rgb8> = unsafe{ screenshot.unwrap().as_raw_slice() }
	///									.into_iter()
	///									.map(|bgr| Rgb8::from(bgr))
	///									.collect();
	/// }
	/// ```
	pub unsafe fn as_raw_slice<'a>(&self) -> &'a [Bgr8] {
		let blob_length = ((*self._image).width * (*self._image).height) as usize;
		std::slice::from_raw_parts((*self._image).data as *const Bgr8, blob_length)
	}

	#[cfg(feature = "image")]
	/// Saves the screenshot to file
	///
	///	You can save as any filetype that the `image`-crate supports
	///
	/// ```rust
	/// # use rxscreen::Display;
	/// if let Ok(display) = Display::new(":0.0") {
	///		let screenshot = display.screenshot();
	///		screenshot.unwrap().save_as("./screenshot.png");
	/// }
	/// ```
	///
	/// # Warning
	///
	/// **Without** configuring any opt-level (debug build) this function may take upwards of 10 seconds for one screenshot
	/// (measured on debug build, 5760x1080 screenshot, ryzen 7 2700x, gtx 1050 ti)
	///
	/// **With opt-level 3** it takes 0.2 seconds for a screenshot of the same size.
	/// 

	pub fn save_as(self, file: impl Into<PathBuf>) -> std::io::Result<()> {
		use image::{save_buffer, ColorType};
		// Restructure buffer to fit RGB instead of BGRP
		let (width, height) = (self.width, self.height);

		fn thrice<T>(first: T, second: T, third: T) -> impl Iterator<Item = T> {
			use std::iter::once;
			once(first).chain(once(second)).chain(once(third))
		}

		let buffer = unsafe { self.as_raw_slice() };
		let buffer = buffer.into_iter().map(|brg| thrice(brg.r, brg.g, brg.b)).flatten().collect::<Vec<u8>>();

		match save_buffer(file.into(), &buffer, width, height, ColorType::Rgb8) {
			Ok(()) => Ok(()),
			Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Couldn't write to file"))
		}
	}

	#[cfg(feature = "save")]
	/// Saves the raw screenshot to PNG in memory
	///
	/// ```rust
	///	# use rxscreen::{Display, Image};
	/// if let Ok(display) = Display::new(":0.0") {
	///		let screenshot = display.screenshot().unwrap();
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
		let (width, height) = (self.width, self.height);
		encoder.encode(&buffer, width, height, ColorType::Rgb8);
		
		Ok(png_data)
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		unsafe{ crate::XDestroyImage(self._image); }
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