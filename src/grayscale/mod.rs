//! This module describes an interleaved single component grayscale buffer

use crate::*;
pub use grapho_color::DigitalGrayscaleColor;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GrayscaleComponent {
    Value = 0,
    Ignore = 1
}

/// Grayscale Pixel buffer without alpha channel
#[derive(Debug, PartialEq)]
pub struct GrayscalePixelBuffer<'a> {
    width: usize,
    height: usize,
    stride: usize,
    fourcc: &'a str,
    component_order: Vec<GrayscaleComponent>,
    data: Vec<u8>
}

impl<'a> GrayscalePixelBuffer<'a> {
    fn decode_component_order(fourcc:&'a str) -> Vec<GrayscaleComponent> {
        match fourcc {
            "Y" => vec![GrayscaleComponent::Value],
            "Yxx" => vec![GrayscaleComponent::Value, GrayscaleComponent::Ignore, GrayscaleComponent::Ignore],
            "Yx" => vec![GrayscaleComponent::Value, GrayscaleComponent::Ignore],
            "xY" => vec![GrayscaleComponent::Ignore, GrayscaleComponent::Value],
            _ => vec![GrayscaleComponent::Value]
        }       
    }
}

impl<'a> PixelBuffer<'a> for GrayscalePixelBuffer<'a> {
    type ColorType = DigitalGrayscaleColor;

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
    /// * `Y`, tightly packed grayscale only image
    /// * `Yxx`, grayscale image with 2 bytes of padding (to interpret a YUV444 image as grayscale)
    /// * `Yx`, grayscale image with 1 byte of padding (to interpret a YUV422 interleaved image as grayscale)
    /// * `xY`, like `Yx` but inverted order
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `GrayscalePixelBuffer` with it's contents set to zero
    fn new(width: usize, height: usize, stride: Option<usize>, fourcc: Option<&'a str>) -> Self {
        let f = fourcc.unwrap_or("Y");
        let component_order = GrayscalePixelBuffer::decode_component_order(f);
        let line_width = stride.unwrap_or(width * component_order.len());

        GrayscalePixelBuffer {
            width,
            height,
            stride: line_width,
            fourcc: f,
            component_order,
            data: vec![0; line_width * height]
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
    /// * `Y`, tightly packed grayscale only image
    /// * `Yxx`, grayscale image with 2 bytes of padding (to interpret a YUV444 image as grayscale)
    /// * `Yx`, grayscale image with 1 byte of padding (to interpret a YUV422 interleaved image as grayscale)
    /// * `xY`, like `Yx` but inverted order
    /// 
    /// # Returns
    /// 
    /// This returns a `Result` with either a new instance of `GrayscalePixelBuffer`
    /// or `PixelBufferError::BufferTooSmall` if the buffer is too small for
    /// the requested dimensions
    fn new_with_data(width: usize, height: usize, data: Vec<u8>, stride: Option<usize>, fourcc: Option<&'a str>) -> Result<Self, PixelBufferError> {

        if data.len() < stride.unwrap_or(width) * height {
            return Err(PixelBufferError::BufferTooSmall);
        }
        
        let f = fourcc.unwrap_or("Y");
        let component_order = GrayscalePixelBuffer::decode_component_order(f);

        Ok(
            GrayscalePixelBuffer {
                width,
                height,
                stride: stride.unwrap_or(width),
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
    /// * `Y`, tightly packed grayscale only image (default)
    /// * `Yxx`, grayscale image with 2 bytes of padding (to interpret a YUV444 image as grayscale)
    /// * `Yx`, grayscale image with 1 byte of padding (to interpret a YUV422 interleaved image as grayscale)
    /// * `xY`, like `Yx` but inverted order
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `GrayscalePixelBuffer` with it's contents set to the
    /// defined color. If stride is bigger than needed width the padding is filled with
    /// zeroes.
    fn new_with_background(width: usize, height: usize, color: Self::ColorType, stride: Option<usize>, fourcc: Option<&'a str>) -> Self {
        let f = fourcc.unwrap_or("Y");
        let component_order = GrayscalePixelBuffer::decode_component_order(f);
        let line_width = stride.unwrap_or(width * component_order.len());
        let data:Vec<u8>;

        let mut representation:Vec<u8> = vec![0; component_order.len()];
        for i in 0..component_order.len() {
            representation[i] =
                match component_order[i] {
                    GrayscaleComponent::Value => color.v,
                    GrayscaleComponent::Ignore => 0u8
                };
        }
        if line_width > width {
            let mut line = representation.repeat(width);
            line.extend([0].repeat(line_width - width * component_order.len()));
            data = line.repeat(height);
        } else {
             data = representation.repeat(width * height);
        }  
         
        GrayscalePixelBuffer {
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

        let start = x * self.component_order.len() + y * self.stride;
        for i in 0..self.component_order.len() {
            match self.component_order[i] {
                GrayscaleComponent::Value => self.data[start + i] = color.v,
                GrayscaleComponent::Ignore => ()
            }
        }

        Ok(())
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<Self::ColorType, PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let mut color = DigitalGrayscaleColor { v: 0 };
        let start = x + y * self.stride;
        for i in 0..self.component_order.len() {
            match self.component_order[i] {
                GrayscaleComponent::Value => {
                    color = DigitalGrayscaleColor::from(self.data[start + i]);
                }
                GrayscaleComponent::Ignore => ()
            }
        }
        
        Ok(color)
    }
}

pub mod iter;
mod tests;
