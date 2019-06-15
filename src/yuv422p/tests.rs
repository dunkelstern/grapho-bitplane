//
// Tests
//
pub use crate::{ PixelBuffer, PixelBufferError };
pub use crate::yuv422p::YUV422pPixelBuffer;
pub use grapho_color::DigitalYCbCrColor;

#[cfg(test)]
mod iter {
    use super::*;

    #[test]
    fn iter_buffer() {
        let color = DigitalYCbCrColor{ y: 255, cb: 64, cr: 0 };
        let buffer = YUV422pPixelBuffer::new_with_background(2, 2, color, None, Some("YV12"));

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}

#[cfg(test)]
mod yv12 {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV422pPixelBuffer::new(2, 2, None, Some("YV12"));
        assert_eq!(buffer.data.len(), 8);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![255, 64, 128, 64, 255, 64, 128, 64];
        let copy = data.clone();
        let buffer = YUV422pPixelBuffer::new_with_data(2, 2, data, None, Some("YV12")).unwrap();
        assert_eq!(buffer.data.len(), 8);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV422pPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            None, Some("YV12")
        );
        assert_eq!(buffer.data.len(), 8);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[y * 2 + x + 0], 64, "Y, x: {}, y: {}", x, y);
                assert_eq!(buffer.data[4 + y + x / 2], 255, "U, x: {}, y: {}", x, y);
                assert_eq!(buffer.data[6 + y + x / 2], 128, "V, x: {}, y: {}", x, y);
            }
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = YUV422pPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            Some(4), Some("YV12")
        );

        assert_eq!(buffer.data.len(), 16);
        for y in 0..2 {
            assert_eq!(buffer.data[y * 4 + 0], 64, "Y, x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 4 + 1], 64, "Y, x: {}, y: {}", 2, y);

            assert_eq!(buffer.data[8 + y * 2 + 0], 255, "U, x: 1/2, y: {}", y);

            assert_eq!(buffer.data[12 + y * 2 + 0], 128, "V, x: 1/2, y: {}", y);
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = YUV422pPixelBuffer::new(2, 2, None, Some("YV12"));
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        println!("Buffer: {:?}", buffer.data);
        assert_eq!(buffer.data[3], 64);
        assert_eq!(buffer.data[5], 127);
        assert_eq!(buffer.data[7], 64);
    }

    #[test]
    fn get_pixel() {
        let data = vec![
            1, 2, 0, 0,  // Y, y = 0
            3, 4, 0, 0,  // Y, y = 1
            5, 0,        // U, y = 0
            6, 0,        // U, y = 1
            7, 0,        // V, y = 0 
            8, 0,        // V, y = 1
        ];
        let buffer = YUV422pPixelBuffer::new_with_data(2, 2, data, Some(4), Some("YV12")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 1, cb: 5, cr: 7 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 4, cb: 6, cr: 8 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}

#[cfg(test)]
mod yv21 {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = YUV422pPixelBuffer::new(2, 2, None, Some("YV21"));
        assert_eq!(buffer.data.len(), 8);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.data[1], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![255, 64, 128, 64, 255, 64, 128, 64];
        let copy = data.clone();
        let buffer = YUV422pPixelBuffer::new_with_data(2, 2, data, None, Some("YV21")).unwrap();
        assert_eq!(buffer.data.len(), 8);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = YUV422pPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            None, Some("YV21")
        );
        assert_eq!(buffer.data.len(), 8);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[y * 2 + x + 0], 64, "Y, x: {}, y: {}", x, y);
                assert_eq!(buffer.data[4 + y + x / 2], 128, "V, x: {}, y: {}", x, y);
                assert_eq!(buffer.data[6 + y + x / 2], 255, "U, x: {}, y: {}", x, y);
            }
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = YUV422pPixelBuffer::new_with_background(
            2, 2,
            DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 },
            Some(4), Some("YV21")
        );

        assert_eq!(buffer.data.len(), 16);
        for y in 0..2 {
            assert_eq!(buffer.data[y * 4 + 0], 64, "Y, x: {}, y: {}", 1, y);
            assert_eq!(buffer.data[y * 4 + 1], 64, "Y, x: {}, y: {}", 2, y);

            assert_eq!(buffer.data[8 + y * 2 + 0], 128, "V, x: 1/2, y: {}", y);

            assert_eq!(buffer.data[12 + y * 2 + 0], 255, "U, x: 1/2, y: {}", y);
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = YUV422pPixelBuffer::new(2, 2, None, Some("YV21"));
        
        match buffer.set_pixel(1, 1, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalYCbCrColor{ y: 64, cb: 255, cr: 128 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[3], 64);
        assert_eq!(buffer.data[5], 64);
        assert_eq!(buffer.data[7], 127);
    }

    #[test]
    fn get_pixel() {
        let data = vec![
            1, 2, 0, 0,  // Y, y = 0
            3, 4, 0, 0,  // Y, y = 1
            5, 0,        // U, y = 0
            6, 0,        // U, y = 1
            7, 0,        // V, y = 0 
            8, 0,        // V, y = 1
        ];
        let buffer = YUV422pPixelBuffer::new_with_data(2, 2, data, Some(4), Some("YV21")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 1, cb: 7, cr: 5 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalYCbCrColor{ y: 4, cb: 8, cr: 6 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}
