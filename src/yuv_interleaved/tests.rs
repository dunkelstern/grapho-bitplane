//
// Tests
//
pub use crate::{ PixelBuffer, PixelBufferError };
pub use crate::yuv_interleaved::YUV422iPixelBuffer;
pub use grapho_color::DigitalYCbCrColor;

#[cfg(test)]
mod iter {
    use super::*;

    #[test]
    fn iter_buffer() {
        let color = DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 };
        let buffer = YUV422iPixelBuffer::new_with_background(2, 2, color, None, Some("YUV422"));

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}

#[cfg(test)]
mod uyvy {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV422iPixelBuffer::new(2, 2, None, Some("UYVY"));
        assert_eq!(buffer.data.len(), 8);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 2);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 2);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![255, 64, 128, 64, 255, 64, 128, 64];
        let copy = data.clone();
        let buffer = YUV422iPixelBuffer::new_with_data(2, 2, data, None, Some("UYVY")).unwrap();
        assert_eq!(buffer.data.len(), 8);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV422iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            None, Some("UYVY")
        );
        assert_eq!(buffer.data.len(), 8);
        for x in (0..8).step_by(4) {
            assert_eq!(buffer.data[x + 0], 255);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 2], 128);
            assert_eq!(buffer.data[x + 3], 64);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = YUV422iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            Some(12), Some("UYVY")
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            assert_eq!(buffer.data[y * 12 + 0], 255, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 1], 64, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 2], 128, "x: {}, y: {}", 2, y);
            assert_eq!(buffer.data[y * 12 + 3], 64, "x: {}, y: {}", 2, y);
            for p in 4..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = YUV422iPixelBuffer::new(2, 2, None, Some("UYVY"));
        assert_eq!(buffer.data[6], 0);
        assert_eq!(buffer.data[7], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[4], 127);
        assert_eq!(buffer.data[5], 0);
        assert_eq!(buffer.data[6], 64);
        assert_eq!(buffer.data[7], 64);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 0, 0, 0, 0, 4, 5, 6, 7, 0, 0, 0, 0];
        let buffer = YUV422iPixelBuffer::new_with_data(2, 2, data, Some(8), Some("UYVY")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 1, cb: 0, cr: 2 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 7, cb: 4, cr: 6 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

#[cfg(test)]
mod yvyu {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV422iPixelBuffer::new(2, 2, None, Some("YVYU"));
        assert_eq!(buffer.data.len(), 8);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 2);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 2);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![255, 64, 128, 64, 255, 64, 128, 64];
        let copy = data.clone();
        let buffer = YUV422iPixelBuffer::new_with_data(2, 2, data, None, Some("YVYU")).unwrap();
        assert_eq!(buffer.data.len(), 8);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV422iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            None, Some("YVYU")
        );
        assert_eq!(buffer.data.len(), 8);
        for x in (0..8).step_by(4) {
            assert_eq!(buffer.data[x + 0], 64);
            assert_eq!(buffer.data[x + 1], 128);
            assert_eq!(buffer.data[x + 2], 64);
            assert_eq!(buffer.data[x + 3], 255);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = YUV422iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            Some(12), Some("YVYU")
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            assert_eq!(buffer.data[y * 12 + 0], 64, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 1], 128, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 2], 64, "x: {}, y: {}", 2, y);
            assert_eq!(buffer.data[y * 12 + 3], 255, "x: {}, y: {}", 2, y);
            for p in 4..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = YUV422iPixelBuffer::new(2, 2, None, Some("YVYU"));
        assert_eq!(buffer.data[6], 0);
        assert_eq!(buffer.data[7], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[4], 0);
        assert_eq!(buffer.data[5], 64);
        assert_eq!(buffer.data[6], 64);
        assert_eq!(buffer.data[7], 127);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 0, 0, 0, 0, 4, 5, 6, 7, 0, 0, 0, 0];
        let buffer = YUV422iPixelBuffer::new_with_data(2, 2, data, Some(8), Some("YVYU")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 0, cb: 3, cr: 1 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 6, cb: 7, cr: 5 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

#[cfg(test)]
mod vyuy {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV422iPixelBuffer::new(2, 2, None, Some("VYUY"));
        assert_eq!(buffer.data.len(), 8);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 2);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 2);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![255, 64, 128, 64, 255, 64, 128, 64];
        let copy = data.clone();
        let buffer = YUV422iPixelBuffer::new_with_data(2, 2, data, None, Some("VYUY")).unwrap();
        assert_eq!(buffer.data.len(), 8);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV422iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            None, Some("VYUY")
        );
        assert_eq!(buffer.data.len(), 8);
        for x in (0..8).step_by(4) {
            assert_eq!(buffer.data[x + 0], 128);
            assert_eq!(buffer.data[x + 1], 64);
            assert_eq!(buffer.data[x + 2], 255);
            assert_eq!(buffer.data[x + 3], 64);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = YUV422iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            Some(12), Some("VYUY")
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            assert_eq!(buffer.data[y * 12 + 0], 128, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 1], 64, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 2], 255, "x: {}, y: {}", 2, y);
            assert_eq!(buffer.data[y * 12 + 3], 64, "x: {}, y: {}", 2, y);
            for p in 4..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = YUV422iPixelBuffer::new(2, 2, None, Some("VYUY"));
        assert_eq!(buffer.data[6], 0);
        assert_eq!(buffer.data[7], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[4], 64);
        assert_eq!(buffer.data[5], 0);
        assert_eq!(buffer.data[6], 127);
        assert_eq!(buffer.data[7], 64);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 0, 0, 0, 0, 4, 5, 6, 7, 0, 0, 0, 0];
        let buffer = YUV422iPixelBuffer::new_with_data(2, 2, data, Some(8), Some("VYUY")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 1, cb: 2, cr: 0 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 7, cb: 6, cr: 4 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

#[cfg(test)]
mod yuv422 {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV422iPixelBuffer::new(2, 2, None, Some("YUV422"));
        assert_eq!(buffer.data.len(), 8);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2 * 2);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2 * 2);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![255, 64, 128, 64, 255, 64, 128, 64];
        let copy = data.clone();
        let buffer = YUV422iPixelBuffer::new_with_data(2, 2, data, None, Some("YUV422")).unwrap();
        assert_eq!(buffer.data.len(), 8);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV422iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            None, Some("YUV422")
        );
        assert_eq!(buffer.data.len(), 8);
        for x in (0..8).step_by(4) {
            assert_eq!(buffer.data[x + 0], 64);
            assert_eq!(buffer.data[x + 1], 255);
            assert_eq!(buffer.data[x + 2], 64);
            assert_eq!(buffer.data[x + 3], 128);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = YUV422iPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            Some(12), Some("YUV422")
        );

        assert_eq!(buffer.data.len(), 24);
        for y in 0..2 {
            assert_eq!(buffer.data[y * 12 + 0], 64, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 1], 255, "x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 12 + 2], 64, "x: {}, y: {}", 2, y);
            assert_eq!(buffer.data[y * 12 + 3], 128, "x: {}, y: {}", 2, y);
            for p in 4..12 {
                assert_eq!(buffer.data[p + y * 12], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = YUV422iPixelBuffer::new(2, 2, None, Some("YUV422"));
        assert_eq!(buffer.data[6], 0);
        assert_eq!(buffer.data[7], 0);
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[4], 0);
        assert_eq!(buffer.data[5], 127);
        assert_eq!(buffer.data[6], 64);
        assert_eq!(buffer.data[7], 64);
    }

    #[test]
    fn get_pixel() {
        let data = vec![0, 1, 2, 3, 0, 0, 0, 0, 4, 5, 6, 7, 0, 0, 0, 0];
        let buffer = YUV422iPixelBuffer::new_with_data(2, 2, data, Some(8), Some("YUV422")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 0, cb: 1, cr: 3 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 6, cb: 5, cr: 7 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}
