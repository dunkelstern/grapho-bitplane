//
// Tests
//

pub use crate::{ PixelBuffer, PixelBufferError };
pub use crate::grayscale::GrayscalePixelBuffer;
pub use grapho_color::DigitalGrayscaleColor;

#[cfg(test)]
mod grayscale {
    use super::*;

    #[test]
    fn empty_buffer() {
        let buffer = GrayscalePixelBuffer::new(2, 2, None, Some("Y"));
        assert_eq!(buffer.data.len(), 4);
        assert_eq!(buffer.data[0], 0);
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.stride, 2);
        assert_eq!(buffer.get_width(), 2);
        assert_eq!(buffer.get_height(), 2);
        assert_eq!(buffer.get_stride(), 2);
    }

    #[test]
    fn prefilled_buffer() {
        let data = vec![0, 255, 64, 0];
        let copy = data.clone();
        let buffer = GrayscalePixelBuffer::new_with_data(2, 2, data, None, Some("Y")).unwrap();
        assert_eq!(buffer.data.len(), 4);
        for x in 0..buffer.data.len() {
            assert_eq!(copy[x], buffer.data[x]);
        }
    }

    #[test]
    fn bg_buffer() {
        let buffer = GrayscalePixelBuffer::new_with_background(
            2, 2,
            DigitalGrayscaleColor{ v: 255 },
            None, Some("Y")
        );
        assert_eq!(buffer.data.len(), 4);
        for x in 0..4 {
            assert_eq!(buffer.data[x], 255);
        }
    }
    
    #[test]
    fn bg_buffer_stride() {
        let buffer = GrayscalePixelBuffer::new_with_background(
            2, 2,
            DigitalGrayscaleColor{ v: 255 },
            Some(4), Some("Y")
        );

        assert_eq!(buffer.data.len(), 8);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(buffer.data[x + y * 4 + 0], 255, "x: {}, y: {}", x, y);
            }
            for p in 3..4 {
                assert_eq!(buffer.data[p + y * 4], 0);
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut buffer = GrayscalePixelBuffer::new(2, 2, None, Some("Y"));
        assert_eq!(buffer.data[3], 0);
        
        match buffer.set_pixel(1, 1, DigitalGrayscaleColor{ v: 255 }) {
            Err(_error) => assert!(false),
            _ => assert!(true)
        }

        match buffer.set_pixel(2, 0, DigitalGrayscaleColor{ v: 255 }) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            _ => assert!(false)
        }

        assert_eq!(buffer.data[3], 255);
    }

    #[test]
    fn get_pixel() {
        let data = vec![1, 2, 0, 0, 3, 4, 0, 0];
        let buffer = GrayscalePixelBuffer::new_with_data(2, 2, data, Some(4), Some("Y")).unwrap();

        match buffer.get_pixel(0, 0) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalGrayscaleColor{ v: 1 })
        }

        match buffer.get_pixel(1, 1) {
            Err(_error) => assert!(false),
            Ok(color) => assert_eq!(color, DigitalGrayscaleColor{ v: 4 })
        }

        match buffer.get_pixel(0, 2) {
            Err(error) => assert_eq!(error, PixelBufferError::RequestOutOfBounds),
            Ok(_color) => assert!(false)
        }
    }
}


#[cfg(test)]
mod iter {
    use super::*;

    #[test]
    fn iter_buffer() {
        let color = DigitalGrayscaleColor{ v: 255 };
        let buffer = GrayscalePixelBuffer::new_with_background(2, 2, color, None, Some("Y"));

        for pixel in buffer {
            assert_eq!(pixel.2, color);
        }
    }
}
