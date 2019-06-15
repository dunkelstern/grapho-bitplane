//! Create iterator for RGBPixelBuffer

use crate::{ Pixel, PixelBuffer };
use super::YUV444iPixelBuffer;
use grapho_color::DigitalYCbCrColor;


/// Pixel iterator for `RGBPixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalRGBColor>`
#[derive(Debug, PartialEq)]
pub struct YUV444iPixelIterator<'a> {
    base: YUV444iPixelBuffer<'a>,
    x: usize,
    y: usize
}

impl<'a> Iterator for YUV444iPixelIterator<'a> {
    type Item = Pixel<DigitalYCbCrColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.base.height {
            return None;
        }

        let color = self.base.get_pixel(self.x, self.y).unwrap();
        self.x += 1;
        if self.x >= self.base.width {
            self.x = 0;
            self.y += 1;
        }

        Some((self.x, self.y, color))
    }

    fn count(self) -> usize {
        self.base.width * self.base.height
    }
}

impl<'a> IntoIterator for YUV444iPixelBuffer<'a> {
    type Item = Pixel<DigitalYCbCrColor>;
    type IntoIter = YUV444iPixelIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        YUV444iPixelIterator {
            base: self,
            x: 0,
            y: 0
        }
    }
}
