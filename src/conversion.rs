//! This module contains bitplane and color conversions
use crate::PixelBuffer;

macro_rules! conversion {
    ($t:ty, $u:ty) => (

        impl From<$t> for $u {

            fn from(f: $t) -> Self {
                let mut buffer:$u = <$u>::new(f.get_width(), f.get_height(), None, None);
                for (x, y, color) in f {
                    buffer.set_pixel(x, y, color.into()).unwrap();
                }

                buffer
            }
        }
    )
}

use crate::rgb::RGBPixelBuffer;
use crate::grayscale::GrayscalePixelBuffer;
use crate::yuv422i::YUV422iPixelBuffer;


conversion!(RGBPixelBuffer<'_>, GrayscalePixelBuffer<'_>);
conversion!(RGBPixelBuffer<'_>, YUV422iPixelBuffer<'_>);

conversion!(YUV422iPixelBuffer<'_>, RGBPixelBuffer<'_>);
conversion!(YUV422iPixelBuffer<'_>, GrayscalePixelBuffer<'_>);

conversion!(GrayscalePixelBuffer<'_>, RGBPixelBuffer<'_>);
conversion!(GrayscalePixelBuffer<'_>, YUV422iPixelBuffer<'_>);
