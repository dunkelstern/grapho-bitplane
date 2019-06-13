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
use crate::y800::Y800PixelBuffer;

conversion!(RGBPixelBuffer, BGRPixelBuffer);
conversion!(RGBPixelBuffer, RGBAPixelBuffer);
conversion!(RGBPixelBuffer, BGRAPixelBuffer);
conversion!(RGBPixelBuffer, Y800PixelBuffer);

conversion!(BGRPixelBuffer, RGBPixelBuffer);
conversion!(BGRPixelBuffer, RGBAPixelBuffer);
conversion!(BGRPixelBuffer, BGRAPixelBuffer);
conversion!(BGRPixelBuffer, Y800PixelBuffer);

conversion!(RGBAPixelBuffer, RGBPixelBuffer);
conversion!(RGBAPixelBuffer, BGRPixelBuffer);
conversion!(RGBAPixelBuffer, BGRAPixelBuffer);
conversion!(RGBAPixelBuffer, Y800PixelBuffer);

conversion!(BGRAPixelBuffer, RGBPixelBuffer);
conversion!(BGRAPixelBuffer, BGRPixelBuffer);
conversion!(BGRAPixelBuffer, RGBAPixelBuffer);
conversion!(BGRAPixelBuffer, Y800PixelBuffer);

conversion!(Y800PixelBuffer, RGBPixelBuffer);
conversion!(Y800PixelBuffer, BGRPixelBuffer);
conversion!(Y800PixelBuffer, RGBAPixelBuffer);
conversion!(Y800PixelBuffer, BGRAPixelBuffer);
