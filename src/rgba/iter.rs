//! Create iterator for RGBAPixelBuffer

use crate::Pixel;
use super::RGBAPixelBuffer;
use grapho_color::DigitalRGBAColor;
use std::convert::TryFrom;


/// Pixel iterator for `RGBAPixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalRGBAColor>`
#[derive(Debug, PartialEq)]
pub struct RGBAPixelIterator {
    base: RGBAPixelBuffer,
    x: usize,
    y: usize
}

impl Iterator for RGBAPixelIterator {
    type Item = Pixel<DigitalRGBAColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.base.height {
            return None;
        }

        let start = self.x * 4 + self.y * self.base.stride;
        let slice = &self.base.data[start..start+4];
        let item = DigitalRGBAColor::try_from(slice).unwrap();
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

impl IntoIterator for RGBAPixelBuffer {
    type Item = Pixel<DigitalRGBAColor>;
    type IntoIter = RGBAPixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        RGBAPixelIterator {
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
    use crate::rgba::RGBAPixelBuffer;
    use grapho_color::DigitalRGBAColor;

    #[test]
    fn iter_rgba_buffer() {
        let color = DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 };
        let buffer = RGBAPixelBuffer::new_with_background(2, 2, None, color);

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}
