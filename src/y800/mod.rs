//! This module describes an interleaved single component grayscale buffer

use crate::*;
pub use grapho_color::DigitalYCbCrColor;

/// Grayscale Pixel buffer without alpha channel
#[derive(Debug, PartialEq)]
pub struct Y800PixelBuffer {
    width: usize,
    height: usize,
    stride: usize,
    data: Vec<u8>
}

impl PixelBuffer for Y800PixelBuffer {
    type ColorType = DigitalYCbCrColor;

    fn new(width: usize, height: usize, stride: Option<usize>) -> Self {
        let line_width = stride.unwrap_or(width);

        Y800PixelBuffer {
            width,
            height,
            stride: line_width,
            data: vec![0; line_width * height]
        }
    }

    fn new_with_data(width: usize, height: usize, stride: Option<usize>, data: Vec<u8>) -> Result<Self, PixelBufferError> {

        if data.len() < stride.unwrap_or(width) * height {
            return Err(PixelBufferError::BufferTooSmall);
        }
        
        Ok(
            Y800PixelBuffer {
                width,
                height,
                stride: stride.unwrap_or(width),
                data
           }
        )
    }

    fn new_with_background(width: usize, height: usize, stride: Option<usize>, color: Self::ColorType) -> Self {
        let line_width = stride.unwrap_or(width);
        let data:Vec<u8>;

        if line_width > width {
            let mut line: Vec<u8> = std::iter::repeat(color.y).take(width).collect();
            line.extend([0].repeat(line_width - width));
            data = line.repeat(height);
        } else {
            data = std::iter::repeat(color.y).take(width * height).collect();;
        }  
         
        Y800PixelBuffer {
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

        let start = x + y * self.stride;
        self.data[start] = color.y;

        Ok(())
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<Self::ColorType, PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let start = x + y * self.stride;
        
        Ok(DigitalYCbCrColor { y: self.data[start], cb: 0, cr: 0 })
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
    fn empty_y800_buffer() {
        let buffer = Y800PixelBuffer::new(2, 2, None);
        assert_eq!(buffer.data.len(), 4);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2);
    }

    #[test]
    fn prefilled_y800_buffer() {
        let data = vec![0, 255, 64, 0];
        let copy = data.clone();
        let buffer = Y800PixelBuffer::new_with_data(2, 2, None, data).unwrap();
        assert_eq!(buffer.data.len(), 4);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_y800_buffer() {
        let buffer = Y800PixelBuffer::new_with_background(
            2, 2, None,
            DigitalYCbCrColor{ y: 255, cb: 0, cr: 0 }
        );
        assert_eq!(buffer.data.len(), 4);
        for x in 0..4 {
            assert_eq!(buffer.data[x], 255);
        }
    }
    
    #[test]
    fn bg_y800_buffer_stride() {
        let buffer = Y800PixelBuffer::new_with_background(
            2, 2, Some(4),
            DigitalYCbCrColor{ y: 255, cb: 0, cr: 0 }
        );

        assert_eq!(buffer.data.len(), 8);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x + y * 4 + 0], 255, "x: {}, y: {}", x, y);
            }
            for p in 3..4 {
                assert_eq!(buffer.data[p + y * 4], 0);
            }
        }
    }

    #[test]
    fn set_pixel_y800_buffer() {
        let mut buffer = Y800PixelBuffer::new(2, 2, None);
        assert_eq!(buffer.data[3], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 255, cr: 0, cb: 0 }.into()) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 255, cr: 0, cb: 0 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[3], 255);
    }

    #[test]
    fn get_pixel_y800_buffer() {
        let data = vec![1, 2, 0, 0, 3, 4, 0, 0];
        let buffer = Y800PixelBuffer::new_with_data(2, 2, Some(4), data).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 1, cb: 0, cr: 0 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 4, cb: 0, cr: 0 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}
