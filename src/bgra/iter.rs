//! Create iterator for BGRAPixelBuffer

use crate::Pixel;
use super::BGRAPixelBuffer;
use grapho_color::DigitalRGBAColor;
use std::convert::TryFrom;


/// Pixel iterator for `BGRAPixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalRGBAColor>`
#[derive(Debug, PartialEq)]
pub struct BGRAPixelIterator {
    base: BGRAPixelBuffer,
    x: usize,
    y: usize
}

impl Iterator for BGRAPixelIterator {
    type Item = Pixel<DigitalRGBAColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.base.height {
            return None;
        }

        let start = self.x * 4 + self.y * self.base.stride;
        let slice = [self.base.data[start + 2], self.base.data[start + 1], self.base.data[start + 0], self.base.data[start + 3]];
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

impl IntoIterator for BGRAPixelBuffer {
    type Item = Pixel<DigitalRGBAColor>;
    type IntoIter = BGRAPixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        BGRAPixelIterator {
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
    use crate::bgra::BGRAPixelBuffer;
    use grapho_color::DigitalRGBAColor;


    #[test]
    fn iter_bgra_buffer() {
        let color = DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 };
        let buffer = BGRAPixelBuffer::new_with_background(2, 2, None, color);

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}
