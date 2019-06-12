//! This module contains bitplane and color conversions
use crate::PixelBuffer;

macro_rules! conversion {
    ($t:ty, $u:ty) => (

        impl From<$t> for $u {

            fn from(f: $t) -> Self {
                let mut buffer:$u = <$u>::new(f.get_width(), f.get_height(), None);
                for (x, y, color) in f {
                    buffer.set_pixel(x, y, color.into()).unwrap();
                }

                buffer
            }
        }
    )
}

use crate::rgb::RGBPixelBuffer;
use crate::bgr::BGRPixelBuffer;
use crate::rgba::RGBAPixelBuffer;
use crate::bgra::BGRAPixelBuffer;

conversion!(RGBPixelBuffer, BGRPixelBuffer);
conversion!(RGBPixelBuffer, RGBAPixelBuffer);
conversion!(RGBPixelBuffer, BGRAPixelBuffer);

conversion!(BGRPixelBuffer, RGBPixelBuffer);
conversion!(BGRPixelBuffer, RGBAPixelBuffer);
conversion!(BGRPixelBuffer, BGRAPixelBuffer);

conversion!(RGBAPixelBuffer, RGBPixelBuffer);
conversion!(RGBAPixelBuffer, BGRPixelBuffer);
conversion!(RGBAPixelBuffer, BGRAPixelBuffer);

conversion!(BGRAPixelBuffer, RGBPixelBuffer);
conversion!(BGRAPixelBuffer, BGRPixelBuffer);
conversion!(BGRAPixelBuffer, RGBAPixelBuffer);
