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

/// Pixel iterator for `RGBPixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalRGBColor>`
#[derive(Debug, PartialEq)]
pub struct RGBPixelIterator {
    base: RGBPixelBuffer,
    x: usize,
    y: usize
}

impl Iterator for RGBPixelIterator {
    type Item = Pixel<DigitalRGBColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.base.height {
            return None;
        }

        let start = self.x * 3 + self.y * self.base.stride;
        let slice = &self.base.data[start..start+3];
        let item = DigitalRGBColor::try_from(slice).unwrap();
        self.x += 1;
        if self.x >= self.base.width {
            self.x = 0;
            self.y += 1;
        }

        Some((self.x, self.y, item))
    }

    fn count(self) -> usize {
        self.base.width * self.base.height
    }
}

impl IntoIterator for RGBPixelBuffer {
    type Item = Pixel<DigitalRGBColor>;
    type IntoIter = RGBPixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        RGBPixelIterator {
            base: self,
            x: 0,
            y: 0
        }
    }
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

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

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
        let buffer = RGBPixelBuffer::new_with_data(2, 2, None, data).unwrap();
        assert_eq!(buffer.data.len(), 12);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 255);
        assert_eq!(buffer.data[2], 64);
        assert_eq!(buffer.data[3], 0);
        assert_eq!(buffer.data[4], 255);
        assert_eq!(buffer.data[5], 64);
        assert_eq!(buffer.data[6], 0);
        assert_eq!(buffer.data[7], 255);
        assert_eq!(buffer.data[8], 64);
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 255);
        assert_eq!(buffer.data[11], 64);
    }

    #[test]
    fn bg_rgb_buffer() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2, None,
            DigitalRGBColor{ r: 255, g: 64, b: 0 }
        );
        assert_eq!(buffer.data.len(), 12);
        assert_eq!(buffer.data[0], 255);
        assert_eq!(buffer.data[1], 64);
        assert_eq!(buffer.data[2], 0);
        assert_eq!(buffer.data[3], 255);
        assert_eq!(buffer.data[4], 64);
        assert_eq!(buffer.data[5], 0);
        assert_eq!(buffer.data[6], 255);
        assert_eq!(buffer.data[7], 64);
        assert_eq!(buffer.data[8], 0);
        assert_eq!(buffer.data[9], 255);
        assert_eq!(buffer.data[10], 64);
        assert_eq!(buffer.data[11], 0);
    }
    
    #[test]
    fn bg_rgb_buffer_stride() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2, Some(12),
            DigitalRGBColor{ r: 255, g: 64, b: 0 }
        );

        assert_eq!(buffer.data.len(), 24);
        assert_eq!(buffer.data[0], 255);
        assert_eq!(buffer.data[1], 64);
        assert_eq!(buffer.data[2], 0);
        assert_eq!(buffer.data[3], 255);
        assert_eq!(buffer.data[4], 64);
        assert_eq!(buffer.data[5], 0);

        assert_eq!(buffer.data[6], 0);
        assert_eq!(buffer.data[7], 0);
        assert_eq!(buffer.data[8], 0);
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 0);

        assert_eq!(buffer.data[12], 255);
        assert_eq!(buffer.data[13], 64);
        assert_eq!(buffer.data[14], 0);
        assert_eq!(buffer.data[15], 255);
        assert_eq!(buffer.data[16], 64);
        assert_eq!(buffer.data[17], 0);

        assert_eq!(buffer.data[18], 0);
        assert_eq!(buffer.data[19], 0);
        assert_eq!(buffer.data[20], 0);
        assert_eq!(buffer.data[21], 0);
        assert_eq!(buffer.data[22], 0);
        assert_eq!(buffer.data[23], 0);
    }

    #[test]
    fn iter_rgb_buffer() {
        let color = DigitalRGBColor{ r: 255, g: 64, b: 0 };
        let buffer = RGBPixelBuffer::new_with_background(2, 2, None, color);

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }

    #[test]
    fn set_pixel_rgb_buffer() {
        let mut buffer = RGBPixelBuffer::new(2, 2, None);
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 0);
        
        match buffer.set_pixel(1, 1, DigitalRGBColor{ r: 255, g: 64, b: 0 }) {
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