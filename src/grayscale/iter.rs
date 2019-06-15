//! Create iterator for GrayscalePixelBuffer

use crate::{ Pixel, PixelBuffer };
use super::GrayscalePixelBuffer;
use grapho_color::DigitalGrayscaleColor;


/// Pixel iterator for `GrayscalePixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalGrayscaleColor>`
#[derive(Debug, PartialEq)]
pub struct GrayscalePixelIterator<'a> {
    base: GrayscalePixelBuffer<'a>,
    x: usize,
    y: usize
}

impl<'a> Iterator for GrayscalePixelIterator<'a> {
    type Item = Pixel<DigitalGrayscaleColor>;

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

impl<'a> IntoIterator for GrayscalePixelBuffer<'a> {
    type Item = Pixel<DigitalGrayscaleColor>;
    type IntoIter = GrayscalePixelIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GrayscalePixelIterator {
            base: self,
            x: 0,
            y: 0
        }
    }
}
