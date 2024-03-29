//! Create iterator for RGBPixelBuffer

use crate::{ Pixel, PixelBuffer };
use super::RGBPixelBuffer;
use grapho_color::DigitalRGBAColor;


/// Pixel iterator for `RGBPixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalRGBColor>`
#[derive(Debug, PartialEq)]
pub struct RGBPixelIterator<'a> {
    base: RGBPixelBuffer<'a>,
    x: usize,
    y: usize
}

impl<'a> Iterator for RGBPixelIterator<'a> {
    type Item = Pixel<DigitalRGBAColor>;

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

impl<'a> IntoIterator for RGBPixelBuffer<'a> {
    type Item = Pixel<DigitalRGBAColor>;
    type IntoIter = RGBPixelIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RGBPixelIterator {
            base: self,
            x: 0,
            y: 0
        }
    }
}
