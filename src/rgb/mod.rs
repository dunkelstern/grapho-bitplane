//! This module describes an interleaved RGB pixel buffer
//!
//! In memory representation is (one byte each): R, G, B, R, G, B, ...

use crate::*;
pub use grapho_color::DigitalRGBColor;
use std::convert::TryFrom;

/// RGB Pixel buffer without alpha channel
#[derive(Debug, PartialEq)]
pub struct RGBPixelBuffer {
    width: usize,
    height: usize,
    stride: usize,
    data: Vec<u8>
}

impl PixelBuffer for RGBPixelBuffer {
    type ColorType = DigitalRGBColor;

    fn new(width: usize, height: usize, stride: Option<usize>) -> Self {
        let line_width = stride.unwrap_or(width * 3);

        RGBPixelBuffer {
            width,
            height,
            stride: line_width,
            data: vec![0; line_width * height]
        }
    }

    fn new_with_data(width: usize, height: usize, stride: Option<usize>, data: Vec<u8>) -> Result<Self, PixelBufferError> {

        if data.len() < stride.unwrap_or(width * 3) * height {
            return Err(PixelBufferError::BufferTooSmall);
        }
        
        Ok(
            RGBPixelBuffer {
                width,
                height,
                stride: stride.unwrap_or(width * 3),
                data
           }
        )
    }

    fn new_with_background(width: usize, height: usize, stride: Option<usize>, color: Self::ColorType) -> Self {
        let representation: [u8; 3] = color.into();
        let line_width = stride.unwrap_or(width * 3);
        let data:Vec<u8>;

        if line_width > width * 3 {
            let mut line = representation.repeat(width);
            line.extend([0].repeat(line_width - width * 3));
            data = line.repeat(height);
        } else {
            data = representation.repeat(width * height);
        }  
         
        RGBPixelBuffer {
            width,
            height,
            stride: line_width,
            data
        }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_stride(&self) -> usize {
        self.stride
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Self::ColorType) -> Result<(), PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let start = x * 3 + y * self.stride;
        let repr: [u8; 3] = color.into();

        self.data[start + 0] = repr[0];
        self.data[start + 1] = repr[1];
        self.data[start + 2] = repr[2]; 

        Ok(())
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<Self::ColorType, PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let start = x * 3 + y * self.stride;
        let slice = &self.data[start..start+3];
        
        Ok(DigitalRGBColor::try_from(slice).unwrap())
    }
}

pub mod iter;

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;
    use grapho_color::DigitalRGBAColor;

    #[test]
    fn empty_rgb_buffer() {
        let buffer = RGBPixelBuffer::new(2, 2, None);
        assert_eq!(buffer.data.len(), 12);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.data[2], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 3);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 3);
    }

    #[test]
    fn prefilled_rgb_buffer() {
        let data = vec![0, 255, 64, 0, 255, 64, 0, 255, 64, 0, 255, 64];
        let copy = data.clone();
        let buffer = RGBPixelBuffer::new_with_data(2, 2, None, data).unwrap();
        assert_eq!(buffer.data.len(), 12);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_rgb_buffer() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2, None,
            DigitalRGBColor{ r: 255, g: 64, b: 0 }
        );
        assert_eq!(buffer.data.len(), 12);
        for x in (0..12).step_by(3) {
            assert_eq!(buffer.data[x + 0], 255);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 2], 0);
        }
    }
    
    #[test]
    fn bg_rgb_buffer_stride() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2, Some(12),
            DigitalRGBColor{ r: 255, g: 64, b: 0 }
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x * 3 + y * 12 + 0], 255, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 1], 64, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 2], 0, "x: {}, y: {}", x, y);
            }
            for p in 6..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel_rgb_buffer() {
        let mut buffer = RGBPixelBuffer::new(2, 2, None);
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 0);
        
        match buffer.set_pixel(1, 1, DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 }.into()) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalRGBColor{ r: 255, g: 64, b: 0 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[9], 255);
        assert_eq!(buffer.data[10], 64);
        assert_eq!(buffer.data[11], 0);
    }

    #[test]
    fn get_pixel_rgb_buffer() {
        let data = vec![0, 1, 2, 3, 4, 5, 0, 0, 6, 7, 8, 9, 10, 11, 0, 0];
        let buffer = RGBPixelBuffer::new_with_data(2, 2, Some(8), data).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBColor{ r: 0, g: 1, b: 2 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBColor{ r: 9, g: 10, b: 11 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}
