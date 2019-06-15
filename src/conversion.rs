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
use crate::yuv444i::YUV444iPixelBuffer;


conversion!(RGBPixelBuffer<'_>, GrayscalePixelBuffer<'_>);
conversion!(RGBPixelBuffer<'_>, YUV422iPixelBuffer<'_>);
conversion!(RGBPixelBuffer<'_>, YUV444iPixelBuffer<'_>);

conversion!(YUV422iPixelBuffer<'_>, RGBPixelBuffer<'_>);
conversion!(YUV422iPixelBuffer<'_>, GrayscalePixelBuffer<'_>);
conversion!(YUV422iPixelBuffer<'_>, YUV444iPixelBuffer<'_>);

conversion!(GrayscalePixelBuffer<'_>, RGBPixelBuffer<'_>);
conversion!(GrayscalePixelBuffer<'_>, YUV422iPixelBuffer<'_>);
conversion!(GrayscalePixelBuffer<'_>, YUV444iPixelBuffer<'_>);

conversion!(YUV444iPixelBuffer<'_>, RGBPixelBuffer<'_>);
conversion!(YUV444iPixelBuffer<'_>, YUV422iPixelBuffer<'_>);
conversion!(YUV444iPixelBuffer<'_>, GrayscalePixelBuffer<'_>);
