//! Create iterator for RGBPixelBuffer

use crate::Pixel;
use super::UYVYPixelBuffer;
use grapho_color::DigitalYCbCrColor;


/// Pixel iterator for `UVYVPixelBuffer`
/// 
/// Items it will generate are of type `Pixel<DigitalCrCbColor>`
#[derive(Debug, PartialEq)]
pub struct UYVYPixelIterator {
    base: UYVYPixelBuffer,
    x: usize,
    y: usize
}

impl Iterator for UYVYPixelIterator {
    type Item = Pixel<DigitalYCbCrColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.base.height {
            return None;
        }

        let start = self.x * 2 + self.y * self.base.stride;
 
        let item: DigitalYCbCrColor;
        if self.x % 2 == 0 {        
            item = DigitalYCbCrColor {
                y: self.base.data[start + 1],
                cb: self.base.data[start + 0],
                cr: self.base.data[start + 2]
            };
        } else {
            item = DigitalYCbCrColor {
                y: self.base.data[start + 1],
                cb: self.base.data[start - 2],
                cr: self.base.data[start + 0]
            };
        }

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

impl IntoIterator for UYVYPixelBuffer {
    type Item = Pixel<DigitalYCbCrColor>;
    type IntoIter = UYVYPixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        UYVYPixelIterator {
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
    use crate::uyvy::UYVYPixelBuffer;
    use grapho_color::DigitalYCbCrColor;

    #[test]
    fn iter_buffer() {
        let color = DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 };
        let buffer = UYVYPixelBuffer::new_with_background(2, 2, None, color);

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}
