//! This module describes an interleaved YUV pixel buffer with color subsampling (half horizontal resolution)
//!
//! In memory representation is (one byte each): U(0/1), Y0, V(0/1), Y1, ...

use crate::*;
pub use grapho_color::DigitalYCbCrColor;

/// YUV Pixel buffer without alpha channel, half resolution color subsampling
#[derive(Debug, PartialEq)]
pub struct UYVYPixelBuffer {
    width: usize,
    height: usize,
    stride: usize,
    data: Vec<u8>
}

impl PixelBuffer for UYVYPixelBuffer {
    type ColorType = DigitalYCbCrColor;

    fn new(width: usize, height: usize, stride: Option<usize>) -> Self {
        let line_width = stride.unwrap_or(width * 2);

        UYVYPixelBuffer {
            width,
            height,
            stride: line_width,
            data: vec![0; line_width * height]
        }
    }

    fn new_with_data(width: usize, height: usize, stride: Option<usize>, data: Vec<u8>) -> Result<Self, PixelBufferError> {

        if data.len() < stride.unwrap_or(width * 2) * height {
            return Err(PixelBufferError::BufferTooSmall);
        }
        
        Ok(
            UYVYPixelBuffer {
                width,
                height,
                stride: stride.unwrap_or(width * 3),
                data
           }
        )
    }

    fn new_with_background(width: usize, height: usize, stride: Option<usize>, color: Self::ColorType) -> Self {
        let representation = vec![color.cb, color.y, color.cr, color.y];
        let line_width = stride.unwrap_or(width * 2);
        let data:Vec<u8>;

        if line_width > width * 2 {
            let mut line = representation.repeat(width / 2);
            line.extend([0].repeat(line_width - width * 2));
            data = line.repeat(height);
        } else {
            data = representation.repeat(width / 2 * height);
        }  
         
        UYVYPixelBuffer {
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

        let start = x * 2 + y * self.stride;
        let repr: [u8; 3] = color.into();

        if x % 2 == 0 {
            self.data[start + 0] = (self.data[start + 0] + repr[1]) >> 1;
            self.data[start + 1] = repr[0];
            self.data[start + 2] = (self.data[start + 2] + repr[2]) >> 1; 
        } else {
            self.data[start - 2] = (self.data[start - 2] + repr[1]) >> 1;
            self.data[start + 0] = (self.data[start + 0] + repr[2]) >> 1; 
            self.data[start + 1] = repr[0];
        }

        Ok(())
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<Self::ColorType, PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let start = x * 2 + y * self.stride;

        if x % 2 == 0 {        
            Ok(DigitalYCbCrColor {
                y: self.data[start + 1],
                cb: self.data[start + 0],
                cr: self.data[start + 2]
            })
        } else {
            Ok(DigitalYCbCrColor {
                y: self.data[start + 1],
                cb: self.data[start - 2],
                cr: self.data[start + 0]
            })
        }
    }
}

pub mod iter;

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;
    use grapho_color::DigitalYCbCrColor;

    #[test]
    fn empty_buffer() {
        let buffer = UYVYPixelBuffer::new(2, 2, None);
        assert_eq!(buffer.data.len(), 8);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 2);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 2);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![255, 64, 128, 64, 255, 64, 128, 64];
        let copy = data.clone();
        let buffer = UYVYPixelBuffer::new_with_data(2, 2, None, data).unwrap();
        assert_eq!(buffer.data.len(), 8);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = UYVYPixelBuffer::new_with_background(
            2, 2, None,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }
        );
        assert_eq!(buffer.data.len(), 8);
        for x in (0..8).step_by(4) {
            assert_eq!(buffer.data[x + 0], 255);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 2], 128);
            assert_eq!(buffer.data[x + 3], 64);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = UYVYPixelBuffer::new_with_background(
            2, 2, Some(12),
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            assert_eq!(buffer.data[y * 12 + 0], 255, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 1], 64, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 2], 128, "x: {}, y: {}", 2, y);
            assert_eq!(buffer.data[y * 12 + 3], 64, "x: {}, y: {}", 2, y);
            for p in 4..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = UYVYPixelBuffer::new(2, 2, None);
        assert_eq!(buffer.data[6], 0);
        assert_eq!(buffer.data[7], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[4], 127);
        assert_eq!(buffer.data[5], 0);
        assert_eq!(buffer.data[6], 64);
        assert_eq!(buffer.data[7], 64);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 0, 0, 0, 0, 4, 5, 6, 7, 0, 0, 0, 0];
        let buffer = UYVYPixelBuffer::new_with_data(2, 2, Some(8), data).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 1, cb: 0, cr: 2 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 7, cb: 4, cr: 6 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}
