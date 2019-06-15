//! This module describes an interleaved YUV pixel buffer with color subsampling (half horizontal resolution)
//!
//! In memory representation is (one byte each): U(0/1), Y0, V(0/1), Y1, ...

use crate::*;
pub use grapho_color::DigitalYCbCrColor;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum YUVComponent {
    Y = 0,
    U = 1,
    V = 2,
    Ignore = -1
}

/// YUV Pixel buffer without alpha channel, half resolution color subsampling
#[derive(Debug, PartialEq)]
pub struct YUV422iPixelBuffer<'a> {
    width: usize,
    height: usize,
    stride: usize,
    fourcc: &'a str,
    component_order: [Vec<YUVComponent>; 2],
    data: Vec<u8>
}

impl<'a> YUV422iPixelBuffer<'a> {
    fn decode_component_order(fourcc:&'a str) -> [Vec<YUVComponent>; 2] {
        match fourcc {
            "YVYU" => [
                vec![YUVComponent::Y, YUVComponent::V, YUVComponent::Ignore, YUVComponent::U],
                vec![YUVComponent::Ignore, YUVComponent::V, YUVComponent::Y, YUVComponent::U]
            ],
            "UYVY" => [
                vec![YUVComponent::U, YUVComponent::Y, YUVComponent::V, YUVComponent::Ignore],
                vec![YUVComponent::U, YUVComponent::Ignore, YUVComponent::V, YUVComponent::Y]
            ],
            "VYUY" => [
                vec![YUVComponent::V, YUVComponent::Y, YUVComponent::U, YUVComponent::Ignore],
                vec![YUVComponent::V, YUVComponent::Ignore, YUVComponent::U, YUVComponent::Y]
            ],
            "YUYV" | "YUV4:2:2" | "YUV422" | "YUY2" | _ => [
                vec![YUVComponent::Y, YUVComponent::U, YUVComponent::Ignore, YUVComponent::V],
                vec![YUVComponent::Ignore, YUVComponent::U, YUVComponent::Y, YUVComponent::V]
            ],
        }       
    }
}


impl<'a> PixelBuffer<'a> for YUV422iPixelBuffer<'a> {
    type ColorType = DigitalYCbCrColor;

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
    /// # Defined fourcc codes
    /// 
    /// * `YVYU`
    /// * `UYVY`
    /// * `VYUY`
    /// * `YUYV`, `YUV4:2:2`, `YUV422`, `YUY2` (default)
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `YUV422iPixelBuffer` with it's contents set to zero
    fn new(width: usize, height: usize, stride: Option<usize>, fourcc: Option<&'a str>) -> Self {
        let f = fourcc.unwrap_or("YUV422");
        let component_order = YUV422iPixelBuffer::decode_component_order(f);
        let line_width = stride.unwrap_or(width * 2);

        YUV422iPixelBuffer {
            width,
            height,
            stride: line_width,
            data: vec![0; line_width * height],
            fourcc: f,
            component_order
        }
    }

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
    /// # Defined fourcc codes
    /// 
    /// * `YVYU`
    /// * `UYVY`
    /// * `VYUY`
    /// * `YUYV`, `YUV4:2:2`, `YUV422`, `YUY2`
    /// 
    /// # Returns
    /// 
    /// This returns a `Result` with either a new instance of `YUV422iPixelBuffer`
    /// or `PixelBufferError::BufferTooSmall` if the buffer is too small for
    /// the requested dimensions
    fn new_with_data(width: usize, height: usize, data: Vec<u8>, stride: Option<usize>, fourcc: Option<&'a str>) -> Result<Self, PixelBufferError> {

        if data.len() < stride.unwrap_or(width * 2) * height {
            return Err(PixelBufferError::BufferTooSmall);
        }

        let f = fourcc.unwrap_or("YUV422");
        let component_order = YUV422iPixelBuffer::decode_component_order(f);

        Ok(
            YUV422iPixelBuffer {
                width,
                height,
                stride: stride.unwrap_or(width * 3),
                data,
                fourcc: f,
                component_order
           }
        )
    }

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
    /// # Defined fourcc codes
    /// 
    /// * `YVYU`
    /// * `UYVY`
    /// * `VYUY`
    /// * `YUYV`, `YUV4:2:2`, `YUV422`, `YUY2`
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `YUV422iPixelBuffer` with it's contents set to the
    /// defined color. If stride is bigger than needed width the padding is filled with
    /// zeroes.
    fn new_with_background(width: usize, height: usize, color: Self::ColorType, stride: Option<usize>, fourcc: Option<&'a str>) -> Self {
        let f = fourcc.unwrap_or("RGB");
        let component_order = YUV422iPixelBuffer::decode_component_order(f);
        let line_width = stride.unwrap_or(width * 2);
        let data:Vec<u8>;

        let mut representation = vec![0; 4];
        for i in 0..4 {
            representation[i] = 
                match component_order[0][i] {
                    YUVComponent::Y | YUVComponent::Ignore => color.y,
                    YUVComponent::U => color.cb,
                    YUVComponent::V => color.cr
                };
        }

        if line_width > width * 2 {
            let mut line = representation.repeat(width / 2);
            line.extend([0].repeat(line_width - width * 2));
            data = line.repeat(height);
        } else {
            data = representation.repeat(width / 2 * height);
        }  
         
        YUV422iPixelBuffer {
            width,
            height,
            stride: line_width,
            data,
            fourcc: f,
            component_order
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

    fn get_fourcc(&self) -> &'a str {
        self.fourcc
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Self::ColorType) -> Result<(), PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let start = x * 2 + y * self.stride - ((x % 2) * 2);
        let repr: [u8; 3] = color.into();

        let order = &self.component_order[x % 2];
        for i in 0..4 {
            match order[i] {
                YUVComponent::Y => self.data[start + i] = repr[0],
                YUVComponent::U => self.data[start + i] = (self.data[start + i] + repr[1]) >> 1,
                YUVComponent::V => self.data[start + i] = (self.data[start + i] + repr[2]) >> 1,
                YUVComponent::Ignore => ()
            }
        }

        Ok(())
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<Self::ColorType, PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let start = x * 2 + y * self.stride - ((x % 2) * 2);

        let mut y: u8 = 0;
        let mut u: u8 = 0;
        let mut v: u8 = 0;

        let order = &self.component_order[x % 2];
        for i in 0..4 {
            match order[i] {
                YUVComponent::Y => y = self.data[start + i],
                YUVComponent::U => u = self.data[start + i],
                YUVComponent::V => v = self.data[start + i],
                YUVComponent::Ignore => ()
            }
        }

        Ok(DigitalYCbCrColor {
            y, cb: u, cr: v
        })
    }
}

pub mod iter;
mod tests;
