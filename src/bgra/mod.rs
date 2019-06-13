//! This module describes an interleaved RGB pixel buffer
//!
//! In memory representation is (one byte each): B, G, R, A, B, G, R, A...

use crate::*;
pub use grapho_color::DigitalRGBAColor;
use std::convert::TryFrom;

/// RGBA Pixel buffer with alpha channel
#[derive(Debug, PartialEq)]
pub struct BGRAPixelBuffer {
    width: usize,
    height: usize,
    stride: usize,
    data: Vec<u8>
}

impl PixelBuffer for BGRAPixelBuffer {
    type ColorType = DigitalRGBAColor;

    fn new(width: usize, height: usize, stride: Option<usize>) -> Self {
        let line_width = stride.unwrap_or(width * 4);

        BGRAPixelBuffer {
            width,
            height,
            stride: line_width,
            data: vec![0; line_width * height]
        }
    }

    fn new_with_data(width: usize, height: usize, stride: Option<usize>, data: Vec<u8>) -> Result<Self, PixelBufferError> {

        if data.len() < stride.unwrap_or(width * 4) * height {
            return Err(PixelBufferError::BufferTooSmall);
        }
        
        Ok(
            BGRAPixelBuffer {
                width,
                height,
                stride: stride.unwrap_or(width * 4),
                data
           }
        )
    }

    fn new_with_background(width: usize, height: usize, stride: Option<usize>, color: Self::ColorType) -> Self {
        let mut representation: [u8; 4] = color.into();
        let line_width = stride.unwrap_or(width * 4);
        let data:Vec<u8>;

        representation = [representation[2], representation[1], representation[0], representation[3]];

        if line_width > width * 4 {
            let mut line = representation.repeat(width);
            line.extend([0].repeat(line_width - width * 4));
            data = line.repeat(height);
        } else {
            data = representation.repeat(width * height);
        }  
         
        BGRAPixelBuffer {
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

        let start = x * 4 + y * self.stride;
        let repr: [u8; 4] = color.into();

        self.data[start + 2] = repr[0];
        self.data[start + 1] = repr[1];
        self.data[start + 0] = repr[2]; 
        self.data[start + 3] = repr[3]; 

        Ok(())
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<Self::ColorType, PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let start = x * 4 + y * self.stride;
        let slice = [self.data[start + 2], self.data[start + 1], self.data[start + 0], self.data[start + 3]];
        
        Ok(DigitalRGBAColor::try_from(slice).unwrap())
    }
}

pub mod iter;

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;
    use grapho_color::DigitalRGBColor;

    #[test]
    fn empty_buffer() {
        let buffer = BGRAPixelBuffer::new(2, 2, None);
        assert_eq!(buffer.data.len(), 16);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.data[2], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 4);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 4);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![0, 255, 64, 255, 0, 255, 64, 255, 0, 255, 64, 255, 0, 255, 64, 255];
        let copy = data.clone();
        let buffer = BGRAPixelBuffer::new_with_data(2, 2, None, data).unwrap();
        assert_eq!(buffer.data.len(), 16);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = BGRAPixelBuffer::new_with_background(
            2, 2, None,
            DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 }
        );
        assert_eq!(buffer.data.len(), 16);
        for x in (0..16).step_by(4) {
            assert_eq!(buffer.data[x + 2], 255);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 0], 0);
            assert_eq!(buffer.data[x + 3], 255);            
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = BGRAPixelBuffer::new_with_background(
            2, 2, Some(20),
            DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 }
        );

        assert_eq!(buffer.data.len(), 40);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x * 4 + y * 20 + 2], 255, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 4 + y * 20 + 1], 64, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 4 + y * 20 + 0], 0, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 4 + y * 20 + 3], 255, "x: {}, y: {}", x, y);            
            }
            for p in 8..20 {
                assert_eq!(buffer.data[p + y * 20], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = BGRAPixelBuffer::new(2, 2, None);
        assert_eq!(buffer.data[12], 0);
        assert_eq!(buffer.data[13], 0);
        assert_eq!(buffer.data[14], 0);
        assert_eq!(buffer.data[15], 0);
        
        match buffer.set_pixel(1, 1, DigitalRGBColor{ r: 255, g: 64, b: 0 }.into()) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[14], 255);
        assert_eq!(buffer.data[13], 64);
        assert_eq!(buffer.data[12], 0);
        assert_eq!(buffer.data[15], 255);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 8, 9, 10, 11, 12, 13, 14, 15, 0, 0];
        let buffer = BGRAPixelBuffer::new_with_data(2, 2, Some(10), data).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBAColor{ r: 2, g: 1, b: 0, a: 3 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBAColor{ r: 14, g: 13, b: 12, a: 15 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}
