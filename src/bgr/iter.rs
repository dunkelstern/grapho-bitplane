//! Create iterator for BGRPixelBuffer

use crate::Pixel;
use super::BGRPixelBuffer;
use grapho_color::DigitalRGBColor;
use std::convert::TryFrom;


/// Pixel iterator for `BGRPixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalRGBColor>`
#[derive(Debug, PartialEq)]
pub struct BGRPixelIterator {
    base: BGRPixelBuffer,
    x: usize,
    y: usize
}

impl Iterator for BGRPixelIterator {
    type Item = Pixel<DigitalRGBColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.base.height {
            return None;
        }

        let start = self.x * 3 + self.y * self.base.stride;
        let slice = [self.base.data[start + 2], self.base.data[start + 1], self.base.data[start + 0]];
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

impl IntoIterator for BGRPixelBuffer {
    type Item = Pixel<DigitalRGBColor>;
    type IntoIter = BGRPixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        BGRPixelIterator {
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
    use crate::bgr::BGRPixelBuffer;
    use grapho_color::DigitalRGBColor;

    #[test]
    fn iter_buffer() {
        let color = DigitalRGBColor{ r: 255, g: 64, b: 0 };
        let buffer = BGRPixelBuffer::new_with_background(2, 2, None, color);

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }

}
