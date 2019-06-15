//
// Tests
//

pub use crate::{ PixelBuffer, PixelBufferError };
pub use crate::rgb::RGBPixelBuffer;
pub use grapho_color::{ DigitalRGBAColor, DigitalRGBColor };

#[cfg(test)]
mod iter {
    use super::*;

    #[test]
    fn iter_buffer() {
        let color = DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 };
        let buffer = RGBPixelBuffer::new_with_background(2, 2, color, None, Some("RGB"));

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}

#[cfg(test)]
mod rgb {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = RGBPixelBuffer::new(2, 2, None, Some("RGB".into()));
        assert_eq!(buffer.data.len(), 12);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.data[2], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 3);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 3);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![0, 255, 64, 0, 255, 64, 0, 255, 64, 0, 255, 64];
        let copy = data.clone();
        let buffer = RGBPixelBuffer::new_with_data(2, 2, data, None, Some("RGB".into())).unwrap();
        assert_eq!(buffer.data.len(), 12);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2,
            DigitalRGBColor{ r: 255, g: 64, b: 0 }.into(),
            None, Some("RGB")
        );
        assert_eq!(buffer.data.len(), 12);
        for x in (0..12).step_by(3) {
            assert_eq!(buffer.data[x + 0], 255);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 2], 0);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2,
            DigitalRGBColor{ r: 255, g: 64, b: 0 }.into(),
            Some(12), Some("RGB")
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x * 3 + y * 12 + 0], 255, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 1], 64, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 2], 0, "x: {}, y: {}", x, y);
            }
            for p in 6..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = RGBPixelBuffer::new(2, 2, None, Some("RGB"));
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 0);
        
        match buffer.set_pixel(1, 1, DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 }.into()) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalRGBColor{ r: 255, g: 64, b: 0 }.into()) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[9], 255);
        assert_eq!(buffer.data[10], 64);
        assert_eq!(buffer.data[11], 0);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 4, 5, 0, 0, 6, 7, 8, 9, 10, 11, 0, 0];
        let buffer = RGBPixelBuffer::new_with_data(2, 2, data, Some(8), Some("RGB")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBColor{ r: 0, g: 1, b: 2 }.into())
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBColor{ r: 9, g: 10, b: 11 }.into())
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

#[cfg(test)]
mod rgba {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = RGBPixelBuffer::new(2, 2, None, Some("RGBA"));
        assert_eq!(buffer.data.len(), 16);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.data[2], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 4);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 4);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![0, 255, 64, 255, 0, 255, 64, 255, 0, 255, 64, 255, 0, 255, 64, 255];
        let copy = data.clone();
        let buffer = RGBPixelBuffer::new_with_data(2, 2, data, None, Some("RGBA")).unwrap();
        assert_eq!(buffer.data.len(), 16);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2,
            DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 },
            None, Some("RGBA")
        );
        assert_eq!(buffer.data.len(), 16);
        for x in (0..16).step_by(4) {
            assert_eq!(buffer.data[x + 0], 255);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 2], 0);
            assert_eq!(buffer.data[x + 3], 255);            
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2,
            DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 },
            Some(20), Some("RGBA")
        );

        assert_eq!(buffer.data.len(), 40);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x * 4 + y * 20 + 0], 255, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 4 + y * 20 + 1], 64, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 4 + y * 20 + 2], 0, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 4 + y * 20 + 3], 255, "x: {}, y: {}", x, y);            
            }
            for p in 8..20 {
                assert_eq!(buffer.data[p + y * 20], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = RGBPixelBuffer::new(2, 2, None, Some("RGBA"));
        assert_eq!(buffer.data[12], 0);
        assert_eq!(buffer.data[13], 0);
        assert_eq!(buffer.data[14], 0);
        assert_eq!(buffer.data[15], 0);
        
        match buffer.set_pixel(1, 1, DigitalRGBColor{ r: 255, g: 64, b: 0 }.into()) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[12], 255);
        assert_eq!(buffer.data[13], 64);
        assert_eq!(buffer.data[14], 0);
        assert_eq!(buffer.data[15], 255);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 8, 9, 10, 11, 12, 13, 14, 15, 0, 0];
        let buffer = RGBPixelBuffer::new_with_data(2, 2, data, Some(10), Some("RGBA")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBAColor{ r: 0, g: 1, b: 2, a: 3 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBAColor{ r: 12, g: 13, b: 14, a: 15 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}


#[cfg(test)]
mod bgr {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = RGBPixelBuffer::new(2, 2, None, Some("BGR"));
        assert_eq!(buffer.data.len(), 12);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.data[2], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 3);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 3);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![0, 255, 64, 0, 255, 64, 0, 255, 64, 0, 255, 64];
        let copy = data.clone();
        let buffer = RGBPixelBuffer::new_with_data(2, 2, data, None, Some("BGR")).unwrap();
        assert_eq!(buffer.data.len(), 12);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2,
            DigitalRGBColor{ r: 255, g: 64, b: 0 }.into(),
            None, Some("BGR")
        );
        assert_eq!(buffer.data.len(), 12);
        for x in (0..12).step_by(3) {
            assert_eq!(buffer.data[x + 2], 255);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 0], 0);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2,
            DigitalRGBColor{ r: 255, g: 64, b: 0 }.into(),
            Some(12), Some("BGR")
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x * 3 + y * 12 + 2], 255, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 1], 64, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 0], 0, "x: {}, y: {}", x, y);
            }
            for p in 6..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = RGBPixelBuffer::new(2, 2, None, Some("BGR"));
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 0);
        
        match buffer.set_pixel(1, 1, DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalRGBColor{ r: 255, g: 64, b: 0 }.into()) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[11], 255);
        assert_eq!(buffer.data[10], 64);
        assert_eq!(buffer.data[9], 0);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 4, 5, 0, 0, 6, 7, 8, 9, 10, 11, 0, 0];
        let buffer = RGBPixelBuffer::new_with_data(2, 2, data, Some(8), Some("BGR")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBColor{ r: 2, g: 1, b: 0 }.into())
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBColor{ r: 11, g: 10, b: 9 }.into())
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

#[cfg(test)]
mod bgra {
    use super::*;
 
    #[test]
    fn empty_buffer() {
        let buffer = RGBPixelBuffer::new(2, 2, None, Some("BGRA"));
        assert_eq!(buffer.data.len(), 16);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.data[2], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 4);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 4);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![0, 255, 64, 255, 0, 255, 64, 255, 0, 255, 64, 255, 0, 255, 64, 255];
        let copy = data.clone();
        let buffer = RGBPixelBuffer::new_with_data(2, 2, data, None, Some("BGRA")).unwrap();
        assert_eq!(buffer.data.len(), 16);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2,
            DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 },
            None, Some("BGRA")
        );
        assert_eq!(buffer.data.len(), 16);
        for x in (0..16).step_by(4) {
            assert_eq!(buffer.data[x + 2], 255);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 0], 0);
            assert_eq!(buffer.data[x + 3], 255);            
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = RGBPixelBuffer::new_with_background(
            2, 2,
            DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 },
            Some(20), Some("BGRA")
        );

        assert_eq!(buffer.data.len(), 40);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x * 4 + y * 20 + 2], 255, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 4 + y * 20 + 1], 64, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 4 + y * 20 + 0], 0, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 4 + y * 20 + 3], 255, "x: {}, y: {}", x, y);            
            }
            for p in 8..20 {
                assert_eq!(buffer.data[p + y * 20], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = RGBPixelBuffer::new(2, 2, None, Some("BGRA"));
        assert_eq!(buffer.data[12], 0);
        assert_eq!(buffer.data[13], 0);
        assert_eq!(buffer.data[14], 0);
        assert_eq!(buffer.data[15], 0);
        
        match buffer.set_pixel(1, 1, DigitalRGBColor{ r: 255, g: 64, b: 0 }.into()) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalRGBAColor{ r: 255, g: 64, b: 0, a: 255 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[14], 255);
        assert_eq!(buffer.data[13], 64);
        assert_eq!(buffer.data[12], 0);
        assert_eq!(buffer.data[15], 255);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 8, 9, 10, 11, 12, 13, 14, 15, 0, 0];
        let buffer = RGBPixelBuffer::new_with_data(2, 2, data, Some(10), Some("BGRA")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBAColor{ r: 2, g: 1, b: 0, a: 3 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalRGBAColor{ r: 14, g: 13, b: 12, a: 15 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

