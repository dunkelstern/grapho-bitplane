//
// Tests
//

pub use crate::{ PixelBuffer, PixelBufferError };
pub use crate::yuv444i::YUV444iPixelBuffer;
pub use grapho_color::DigitalYCbCrColor;

#[cfg(test)]
mod iter {
    use super::*;

    #[test]
    fn iter_buffer() {
        let color = DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 };
        let buffer = YUV444iPixelBuffer::new_with_background(2, 2, color, None, Some("YUV444"));

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}

#[cfg(test)]
mod yuv {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV444iPixelBuffer::new(2, 2, None, Some("YUV444"));
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
        let buffer = YUV444iPixelBuffer::new_with_data(2, 2, data, None, Some("YUV444")).unwrap();
        assert_eq!(buffer.data.len(), 12);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV444iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 },
            None, Some("YUV444")
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
        let buffer = YUV444iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 },
            Some(12), Some("YUV444")
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
        let mut buffer = YUV444iPixelBuffer::new(2, 2, None, Some("YUV444"));
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 }) {
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
        let buffer = YUV444iPixelBuffer::new_with_data(2, 2, data, Some(8), Some("YUV444")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 0, cb: 1, cr: 2 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 9, cb: 10, cr: 11 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

#[cfg(test)]
mod vuy {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV444iPixelBuffer::new(2, 2, None, Some("VUY"));
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
        let buffer = YUV444iPixelBuffer::new_with_data(2, 2, data, None, Some("VUY")).unwrap();
        assert_eq!(buffer.data.len(), 12);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV444iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 },
            None, Some("VUY")
        );
        assert_eq!(buffer.data.len(), 12);
        for x in (0..12).step_by(3) {
            assert_eq!(buffer.data[x + 0], 0);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 2], 255);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = YUV444iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 },
            Some(12), Some("VUY")
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x * 3 + y * 12 + 0], 0, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 1], 64, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 2], 255, "x: {}, y: {}", x, y);
            }
            for p in 6..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = YUV444iPixelBuffer::new(2, 2, None, Some("VUY"));
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 64);
        assert_eq!(buffer.data[11], 255);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 4, 5, 0, 0, 6, 7, 8, 9, 10, 11, 0, 0];
        let buffer = YUV444iPixelBuffer::new_with_data(2, 2, data, Some(8), Some("VUY")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 2, cb: 1, cr: 0 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 11, cb: 10, cr: 9 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

#[cfg(test)]
mod yvu {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV444iPixelBuffer::new(2, 2, None, Some("YVU"));
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
        let buffer = YUV444iPixelBuffer::new_with_data(2, 2, data, None, Some("YVU")).unwrap();
        assert_eq!(buffer.data.len(), 12);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV444iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 },
            None, Some("YVU")
        );
        assert_eq!(buffer.data.len(), 12);
        for x in (0..12).step_by(3) {
            assert_eq!(buffer.data[x + 0], 255);
            assert_eq!(buffer.data[x + 1], 0);
            assert_eq!(buffer.data[x + 2], 64);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = YUV444iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 },
            Some(12), Some("YVU")
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x * 3 + y * 12 + 0], 255, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 1], 0, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 2], 64, "x: {}, y: {}", x, y);
            }
            for p in 6..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = YUV444iPixelBuffer::new(2, 2, None, Some("YVU"));
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[9], 255);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 64);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 4, 5, 0, 0, 6, 7, 8, 9, 10, 11, 0, 0];
        let buffer = YUV444iPixelBuffer::new_with_data(2, 2, data, Some(8), Some("YVU")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 0, cb: 2, cr: 1 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 9, cb: 11, cr: 10 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

#[cfg(test)]
mod uvy {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV444iPixelBuffer::new(2, 2, None, Some("UVY"));
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
        let buffer = YUV444iPixelBuffer::new_with_data(2, 2, data, None, Some("UVY")).unwrap();
        assert_eq!(buffer.data.len(), 12);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV444iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 },
            None, Some("UVY")
        );
        assert_eq!(buffer.data.len(), 12);
        for x in (0..12).step_by(3) {
            assert_eq!(buffer.data[x + 0], 64);
            assert_eq!(buffer.data[x + 1], 0);
            assert_eq!(buffer.data[x + 2], 255);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = YUV444iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 },
            Some(12), Some("UVY")
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x * 3 + y * 12 + 0], 64, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 1], 0, "x: {}, y: {}", x, y);
                assert_eq!(buffer.data[x * 3 + y * 12 + 2], 255, "x: {}, y: {}", x, y);
            }
            for p in 6..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = YUV444iPixelBuffer::new(2, 2, None, Some("UVY"));
        assert_eq!(buffer.data[9], 0);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[9], 64);
        assert_eq!(buffer.data[10], 0);
        assert_eq!(buffer.data[11], 255);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 4, 5, 0, 0, 6, 7, 8, 9, 10, 11, 0, 0];
        let buffer = YUV444iPixelBuffer::new_with_data(2, 2, data, Some(8), Some("UVY")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 2, cb: 0, cr: 1 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 11, cb: 9, cr: 10 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}
