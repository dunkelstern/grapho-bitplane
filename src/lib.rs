//! `grapho-bitplane` describes the pixel buffer primitives that are used for creating and modifying images.
//! It will contain color conversion functionality and can work on arbitrary interleaved or non-interleaved
//! graphics data.
//! 
//! Currently the following bitplane types are implemented:
//! 
//! - `RGB` interleaved with stride
//! - `BGR` interleaved with stride
//! - `RGBA` interleaved with stride
//! - `BGRA` interleaved with stride
//! 
//! Conversion from all color-plane types into all others will be implemented.


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
pub trait PixelBuffer: Sized + IntoIterator
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
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `Self` with it's contents set to zero
    fn new(width: usize, height: usize, stride: Option<usize>) -> Self;

    /// Create a new pixel buffer with given dimensions from a `Vec<u8>`
    /// 
    /// # Arguments
    /// 
    /// * `width` - The width of the buffer
    /// * `height` - The height of the buffer
    /// * `stride` - optional, the line-width of the buffer if it differs from the
    ///   default: `<length of color type representation> * width`
    /// * `data` - the data to consume
    /// 
    /// # Returns
    /// 
    /// This returns a `Result` with either a new instance of `Self`
    /// or `PixelBufferError::BufferTooSmall` if the buffer is too small for
    /// the requested dimensions
    fn new_with_data(width: usize, height: usize, stride: Option<usize>, data: Vec<u8>) -> Result<Self, PixelBufferError>;
    
    /// Create a new pixel buffer with given dimensions and fill color
    /// 
    /// # Arguments
    /// 
    /// * `width` - The width of the buffer
    /// * `height` - The height of the buffer
    /// * `stride` - optional, the line-width of the buffer if it differs from the
    ///   default: `<length of color type representation> * width`
    /// * `color` - fill color to use
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `Self` with it's contents set to the
    /// defined color. If stride is bigger than needed width the padding is filled with
    /// zeroes.
    fn new_with_background(width: usize, height: usize, stride: Option<usize>, color: Self::ColorType) -> Self;
    
    /// width of the buffer
    fn get_width(&self) -> usize;

    /// height of the buffer
    fn get_height(&self) -> usize;

    /// stride of the buffer
    fn get_stride(&self) -> usize;

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
pub mod rgba;
pub mod bgr;
pub mod bgra;

pub mod conversion;
