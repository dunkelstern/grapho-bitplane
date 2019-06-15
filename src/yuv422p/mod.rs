//! This module describes a planar YUV pixel buffer with color subsampling (half horizontal resolution)

use crate::*;
pub use grapho_color::DigitalYCbCrColor;
pub use crate::yuv422i::YUVComponent;

/// YUV Pixel buffer without alpha channel, half resolution color subsampling
#[derive(Debug, PartialEq)]
pub struct YUV422pPixelBuffer<'a> {
    width: usize,
    height: usize,
    stride: usize,
    fourcc: &'a str,
    component_order: Vec<YUVComponent>,
    data: Vec<u8>
}

impl<'a> YUV422pPixelBuffer<'a> {
    fn decode_component_order(fourcc:&'a str) -> Vec<YUVComponent> {
        match fourcc {
            "YV21" | "YVU" =>
                vec![YUVComponent::V, YUVComponent::U],
            "YV12" | "YUV" | _ =>
                vec![YUVComponent::U, YUVComponent::V],
        }       
    }
}


impl<'a> PixelBuffer<'a> for YUV422pPixelBuffer<'a> {
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
    /// * `YV12`, `YUV` (default)
    /// * `YV21`, `YVU`
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `YUV422pPixelBuffer` with it's contents set to zero
    fn new(width: usize, height: usize, stride: Option<usize>, fourcc: Option<&'a str>) -> Self {
        let f = fourcc.unwrap_or("YV12");
        let component_order = YUV422pPixelBuffer::decode_component_order(f);
        let line_width = stride.unwrap_or(width * 2);

        YUV422pPixelBuffer {
            width,
            height,
            stride: stride.unwrap_or(width),
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
    /// * `YV12`, `YUV` (default)
    /// * `YV21`, `YVU`
    /// 
    /// # Returns
    /// 
    /// This returns a `Result` with either a new instance of `YUV422pPixelBuffer`
    /// or `PixelBufferError::BufferTooSmall` if the buffer is too small for
    /// the requested dimensions
    fn new_with_data(width: usize, height: usize, data: Vec<u8>, stride: Option<usize>, fourcc: Option<&'a str>) -> Result<Self, PixelBufferError> {

        if data.len() < stride.unwrap_or(width) * height {
            return Err(PixelBufferError::BufferTooSmall);
        }

        let f = fourcc.unwrap_or("YV12");
        let component_order = YUV422pPixelBuffer::decode_component_order(f);

        Ok(
            YUV422pPixelBuffer {
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
    /// * `YV12`, `YUV` (default)
    /// * `YV21`, `YVU`
    /// 
    /// # Returns
    /// 
    /// This returns a new instance of `YUV422iPixelBuffer` with it's contents set to the
    /// defined color. If stride is bigger than needed width the padding is filled with
    /// zeroes.
    fn new_with_background(width: usize, height: usize, color: Self::ColorType, stride: Option<usize>, fourcc: Option<&'a str>) -> Self {
        let f = fourcc.unwrap_or("YV12");
        let component_order = YUV422pPixelBuffer::decode_component_order(f);
        let line_width = stride.unwrap_or(width);
        
        let mut data: Vec<u8> = Vec::with_capacity(line_width * height);

        let mut y = [color.y].repeat(width);
        let mut u = [color.cb].repeat(width / 2);
        let mut v = [color.cr].repeat(width / 2);

        if line_width > width {
            y.extend([0].repeat(line_width - width));
            u.extend([0].repeat((line_width - width) / 2));
            v.extend([0].repeat((line_width - width) / 2));
        }

        data.extend(y.repeat(height));

        if component_order[0] == YUVComponent::U {
            data.extend(u.repeat(height));
            data.extend(v.repeat(height));
        } else {
            data.extend(v.repeat(height));
            data.extend(u.repeat(height));
        }
         
        YUV422pPixelBuffer {
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

        let start = x + y * self.stride;
        let plane1 = (self.stride * self.height) + x / 2 + y * self.stride / 2;
        let plane2 = plane1 + (self.stride * self.height / 2);

        self.data[start] = color.y;
        if self.component_order[0] == YUVComponent::U {
            self.data[plane1] = (self.data[plane1] + color.cb) >> 1;
            self.data[plane2] = (self.data[plane2] + color.cr) >> 1;
        } else {
            self.data[plane2] = (self.data[plane2] + color.cb) >> 1;
            self.data[plane1] = (self.data[plane1] + color.cr) >> 1;
        }
   
        Ok(())
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<Self::ColorType, PixelBufferError> {
        if (x >= self.width) || (y >= self.width) {
            return Err(PixelBufferError::RequestOutOfBounds);
        }

        let start = x + y * self.stride;
        let plane1 = (self.stride * self.height) + x / 2 + y * self.stride / 2;
        let plane2 = plane1 + (self.stride * self.height / 2);

        let u: u8;
        let v: u8;

        if self.component_order[0] == YUVComponent::U {
            u = self.data[plane1];
            v = self.data[plane2];
        } else {
            v = self.data[plane1];
            u = self.data[plane2];
        }
 
        Ok(DigitalYCbCrColor {
            y: self.data[start], cb: u, cr: v
        })
    }
}

pub mod iter;
mod tests;
