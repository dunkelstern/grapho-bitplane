//! This module describes an interleaved RGB pixel buffer

use crate::*;
pub use grapho_color::DigitalRGBAColor;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RGBComponent {
    Red   = 0,
    Green = 1,
    Blue  = 2,
    Alpha = 3
}

/// RGB Pixel buffer without alpha channel
#[derive(Debug, PartialEq)]
pub struct RGBPixelBuffer<'a> {
    width: usize,
    height: usize,
    stride: usize,
    fourcc: &'a str,
    component_order: Vec<RGBComponent>,
    data: Vec<u8>
}

impl<'a> RGBPixelBuffer<'a> {
    fn decode_component_order(fourcc:&'a str) -> Vec<RGBComponent> {
        match fourcc {
            "BGR" => vec![RGBComponent::Blue, RGBComponent::Green, RGBComponent::Red],
            "RGBA" => vec![RGBComponent::Red, RGBComponent::Green, RGBComponent::Blue, RGBComponent::Alpha],
            "BGRA" => vec![RGBComponent::Blue, RGBComponent::Green, RGBComponent::Red, RGBComponent::Alpha],
            "ARGB" => vec![RGBComponent::Alpha, RGBComponent::Red, RGBComponent::Green, RGBComponent::Blue],
            "ABGR" => vec![RGBComponent::Alpha, RGBComponent::Blue, RGBComponent::Green, RGBComponent::Red],
            "RGB" | _ => vec![RGBComponent::Red, RGBComponent::Green, RGBComponent::Blue],
        }       
    }
}

impl<'a> PixelBuffer<'a> for RGBPixelBuffer<'a> {
    type ColorType = DigitalRGBAColor;

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
    /// * `RGB`
    /// * `BGR`
    /// * `RGBA`
    /// * `ARGB`
    /// * `BGRA`
    /// * `ABGR`
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `RGBPixelBuffer` with it's contents set to zero

    fn new(width: usize, height: usize, stride: Option<usize>, fourcc: Option<&'a str>) -> Self {
        let f = fourcc.unwrap_or("RGB");
        let component_order = RGBPixelBuffer::decode_component_order(f);
        let line_width = stride.unwrap_or(width * component_order.len());

        RGBPixelBuffer {
            width,
            height,
            data: vec![0; line_width * height],
            stride: line_width,
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
    /// * `RGB`
    /// * `BGR`
    /// * `RGBA`
    /// * `ARGB`
    /// * `BGRA`
    /// * `ABGR`
    /// 
    /// # Returns
    /// 
    /// This returns a `Result` with either a new instance of `RGBPixelBuffer`
    /// or `PixelBufferError::BufferTooSmall` if the buffer is too small for
    /// the requested dimensions
    fn new_with_data(width: usize, height: usize, data: Vec<u8>, stride: Option<usize>, fourcc: Option<&'a str>) -> Result<Self, PixelBufferError> {
        let f = fourcc.unwrap_or("RGB");
        let component_order = RGBPixelBuffer::decode_component_order(f);

        if data.len() < stride.unwrap_or(width * component_order.len()) * height {
            return Err(PixelBufferError::BufferTooSmall);
        }

 
        Ok(
            RGBPixelBuffer {
                width,
                height,
                data,
                stride: stride.unwrap_or(width * component_order.len()),
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
    /// * `RGB` (default)
    /// * `BGR`
    /// * `RGBA`
    /// * `ARGB`
    /// * `BGRA`
    /// * `ABGR`
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `RGBPixelBuffer` with it's contents set to the
    /// defined color. If stride is bigger than needed width the padding is filled with
    /// zeroes.
    fn new_with_background(width: usize, height: usize, color: Self::ColorType, stride: Option<usize>, fourcc: Option<&'a str>) -> Self {
        let f = fourcc.unwrap_or("RGB");
        let component_order = RGBPixelBuffer::decode_component_order(f);
        let rep: [u8; 4] = color.into();
        let line_width = stride.unwrap_or(width * component_order.len());
        let data:Vec<u8>;

        let representation =
            if component_order.len() == 3 {
                vec![
                    rep[component_order[0] as usize],
                    rep[component_order[1] as usize],
                    rep[component_order[2] as usize],
                ]
            } else {
                vec![
                    rep[component_order[0] as usize],
                    rep[component_order[1] as usize],
                    rep[component_order[2] as usize],
                    rep[component_order[3] as usize],
                ]
            };

        if line_width > width * component_order.len() {
            let mut line = representation.repeat(width);
            line.extend([0].repeat(line_width - width * component_order.len()));
            data = line.repeat(height);
        } else {
            data = representation.repeat(width * height);
        }  
         
        RGBPixelBuffer {
            width,
            height,
            data,
            stride: line_width,
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
        let repr: [u8; 4] = color.into();

        for i in 0..self.component_order.len() {
            self.data[start + i] = repr[self.component_order[i] as usize];
        }
 
        Ok(())
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<Self::ColorType, PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let start = x * self.component_order.len() + y * self.stride;
        let mut color: [u8; 4] = [0, 0, 0, 255];
        for i in 0..self.component_order.len() {
            color[self.component_order[i] as usize] = self.data[start + i];
        }
        
        Ok(DigitalRGBAColor::from(color))
    }
}

pub mod iter;
mod tests;
