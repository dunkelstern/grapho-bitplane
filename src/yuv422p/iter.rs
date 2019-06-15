//! Create iterator for YUV422iPixelBuffer

use crate::{ Pixel, PixelBuffer };
use super::YUV422pPixelBuffer;
use grapho_color::DigitalYCbCrColor;


/// Pixel iterator for `YUV422iPixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalCrCbColor>`
#[derive(Debug, PartialEq)]
pub struct YUV422pPixelIterator<'a> {
    base: YUV422pPixelBuffer<'a>,
    x: usize,
    y: usize
}

impl<'a> Iterator for YUV422pPixelIterator<'a> {
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

impl<'a> IntoIterator for YUV422pPixelBuffer<'a> {
    type Item = Pixel<DigitalYCbCrColor>;
    type IntoIter = YUV422pPixelIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        YUV422pPixelIterator {
            base: self,
            x: 0,
            y: 0
        }
    }
}
