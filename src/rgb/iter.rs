//! Create iterator for RGBPixelBuffer

use crate::Pixel;
use super::RGBPixelBuffer;
use grapho_color::DigitalRGBColor;
use std::convert::TryFrom;


/// Pixel iterator for `RGBPixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalRGBColor>`
#[derive(Debug, PartialEq)]
pub struct RGBPixelIterator {
    base: RGBPixelBuffer,
    x: usize,
    y: usize
}

impl Iterator for RGBPixelIterator {
    type Item = Pixel<DigitalRGBColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.base.height {
            return None;
        }

        let start = self.x * 3 + self.y * self.base.stride;
        let slice = &self.base.data[start..start+3];
        let item = DigitalRGBColor::try_from(slice).unwrap();
        self.x += 1;
        if self.x >= self.base.width {
            self.x = 0;
            self.y += 1;
        }

        Some((self.x, self.y, item))
    }

    fn count(self) -> usize {
        self.base.width * self.base.height
    }
}

impl IntoIterator for RGBPixelBuffer {
    type Item = Pixel<DigitalRGBColor>;
    type IntoIter = RGBPixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        RGBPixelIterator {
            base: self,
            x: 0,
            y: 0
        }
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use crate::PixelBuffer;
    use crate::rgb::RGBPixelBuffer;
    use grapho_color::DigitalRGBColor;

    #[test]
    fn iter_buffer() {
        let color = DigitalRGBColor{ r: 255, g: 64, b: 0 };
        let buffer = RGBPixelBuffer::new_with_background(2, 2, None, color);

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}
