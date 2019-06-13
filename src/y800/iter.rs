//! Create iterator for Y800PixelBuffer

use crate::Pixel;
use super::Y800PixelBuffer;
use grapho_color::DigitalYCbCrColor;


/// Pixel iterator for `Y800PixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalYCbCrColor>`
#[derive(Debug, PartialEq)]
pub struct Y800PixelIterator {
    base: Y800PixelBuffer,
    x: usize,
    y: usize
}

impl Iterator for Y800PixelIterator {
    type Item = Pixel<DigitalYCbCrColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.base.height {
            return None;
        }

        let start = self.x + self.y * self.base.stride;
        let item = DigitalYCbCrColor { y: self.base.data[start], cb: 0, cr: 0 };
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

impl IntoIterator for Y800PixelBuffer {
    type Item = Pixel<DigitalYCbCrColor>;
    type IntoIter = Y800PixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        Y800PixelIterator {
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
    use crate::y800::Y800PixelBuffer;
    use grapho_color::DigitalYCbCrColor;

    #[test]
    fn iter_buffer() {
        let color = DigitalYCbCrColor{ y: 255, cb: 0, cr: 0 };
        let buffer = Y800PixelBuffer::new_with_background(2, 2, None, color);

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}
