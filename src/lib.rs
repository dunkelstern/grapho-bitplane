//! `grapho-bitplane` describes the pixel buffer primitives that are used for creating and modifying images.
//! It will contain color conversion functionality and can work on arbitrary interleaved or non-interleaved
//! graphics data.
//! 
//! Conversion from all color-plane types into all others will be implemented.
//! Currently the following bitplane types are implemented:
//! 
//! ### RGB interleaved `RGBPixelBuffer`
//!
//! - `RGB`, `BGR` 24 bit without alpha
//! - `RGBA`, `ARGB`, `BGRA`, `ABGR` 32 bit with alpha
//!
//! ### Grayscale `GrayscalePixelBuffer`
//!
//! - `Y` Simple, single Y plane for monochrome images.
//! - 'Yxx' 3 bytes, ignore the last two (interpret a YUV444 image as grayscale)
//! - 'Yx' and 'xY', 2 bytes, ignore the x (interpret a YUV422 image as grayscale)
//!
//! ### YUV `YUV422iPixelBuffer`
//!
//! - `UYVY` YUV 4:2:2 (Y sample at every pixel, U and V sampled at every second pixel horizontally on each line). A macropixel contains 2 pixels in 1 `u32`.
//! - `YUY2`/`YUV422` YUV 4:2:2 as for `UYVY` but with different component ordering within the `u32` macropixel.
//! - `YVYU` YUV 4:2:2 as for `UYVY` but with different component ordering within the `u32` macropixel.
//! - `VYUY` YUV 4:2:2 as for `UYVY` but with different component ordering within the `u32` macropixel.

#![feature(repeat_generic_slice)]
#![feature(doc_spotlight)]

extern crate grapho_color;

pub use std::ops::{Sub, Mul, Add, Div, SubAssign, MulAssign, AddAssign, DivAssign};
use std::marker::{Sized};

/// Error type for pixel buffer operations
#[derive(Debug, PartialEq)]
pub enum PixelBufferError {
    /// Buffer too small for requested operation
    BufferTooSmall,
    /// Request out of buffer bounds
    RequestOutOfBounds
}

/// Pixel type used by iterators, contains
/// x, y and color of pixel
pub type Pixel<T> = (usize, usize, T);

#[doc(spotlight)]
/// Pixel buffer trait, all Pixel buffers will implement this
pub trait PixelBuffer<'a>: Sized + IntoIterator
    // + Sub + Mul + Add + Div + SubAssign + MulAssign + AddAssign + DivAssign
{
    /// The color type this pixel buffer contains
    type ColorType;

    /// Create a new pixel buffer with given dimensions
    /// 
    /// # Arguments
    /// 
    /// * `width` - The width of the buffer
    /// * `height` - The height of the buffer
    /// * `stride` - optional, the line-width of the buffer if it differs from the
    ///   default: `<length of color type representation> * width`
    /// * `fourcc` - optional, data representation format
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `Self` with it's contents set to zero
    fn new(width: usize, height: usize, stride: Option<usize>, fourcc: Option<&'a str>) -> Self;

    /// Create a new pixel buffer with given dimensions from a `Vec<u8>`
    /// 
    /// # Arguments
    /// 
    /// * `width` - The width of the buffer
    /// * `height` - The height of the buffer
    /// * `stride` - optional, the line-width of the buffer if it differs from the
    ///   default: `<length of color type representation> * width`
    /// * `fourcc` - optional, data representation format
    /// * `data` - the data to consume
    /// 
    /// # Returns
    /// 
    /// This returns a `Result` with either a new instance of `Self`
    /// or `PixelBufferError::BufferTooSmall` if the buffer is too small for
    /// the requested dimensions
    fn new_with_data(width: usize, height: usize, data: Vec<u8>, stride: Option<usize>, fourcc: Option<&'a str>) -> Result<Self, PixelBufferError>;
    
    /// Create a new pixel buffer with given dimensions and fill color
    /// 
    /// # Arguments
    /// 
    /// * `width` - The width of the buffer
    /// * `height` - The height of the buffer
    /// * `stride` - optional, the line-width of the buffer if it differs from the
    ///   default: `<length of color type representation> * width`
    /// * `fourcc` - optional, data representation format
    /// * `color` - fill color to use
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `Self` with it's contents set to the
    /// defined color. If stride is bigger than needed width the padding is filled with
    /// zeroes.
    fn new_with_background(width: usize, height: usize, color: Self::ColorType, stride: Option<usize>, fourcc: Option<&'a str>) -> Self;
    
    /// width of the buffer
    fn get_width(&self) -> usize;

    /// height of the buffer
    fn get_height(&self) -> usize;

    /// stride of the buffer
    fn get_stride(&self) -> usize;

    /// fourcc code of the buffer
    fn get_fourcc(&self) -> &'a str;

    /// Set a pixel to a color
    /// 
    /// # Arguments
    /// 
    /// * `x` - x coordinate (from top left)
    /// * `y` - y coordinate (from top left)
    /// * `color` - color to set the pixel to
    /// 
    /// # Returns
    /// 
    /// A `Result`, either a `()` if everything went ok, or
    /// `PixelBufferError::RequestOutOfBounds` if the request was out of bounds
    fn set_pixel(&mut self, x: usize, y: usize, color: Self::ColorType) -> Result<(), PixelBufferError>;

    /// Get color of pixel at position
    /// 
    /// # Arguments
    /// 
    /// * `x` - x coordinate (from top left)
    /// * `y` - y coordinate (from top left)
    /// 
    /// # Returns
    /// 
    /// A `Result`, either a `ColorType` if everything went ok, or
    /// `PixelBufferError::RequestOutOfBounds` if the request was out of bounds
    fn get_pixel(&self, x: usize, y: usize) -> Result<Self::ColorType, PixelBufferError>;

}

pub mod rgb;
pub mod grayscale;
pub mod yuv422i;
pub mod yuv444i;

pub mod conversion;
