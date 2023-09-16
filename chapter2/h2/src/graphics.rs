/*! Contains facilities to perform visual-related shenanigans
 * 
 */

use std::fs::File;

use num::complex::Complex;
use image::{ColorType, ImageEncoder, codecs::png::PngEncoder};

use crate::mandelbrot;
use crate::units::{SquareBoundsPx, Pixel};


/// Converts a point from Cartesian Mandelbrot
/// to Pixel point on a screen. 
/// 
/// bounds - are pixel image size bounds (width, height)
/// pixel - represents a single pixel with coordinates x, y
/// upper_left, lower_right - designate plane area on mandelbrot set
///
pub fn pixel_to_point(
        bounds: SquareBoundsPx,
        pixel: Pixel,
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
) -> Complex<f64>
{
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im);

    Complex {
        re: (upper_left.re + pixel.x as f64 * width / bounds.width as f64),
        im: (upper_left.im - pixel.y as f64 * height / bounds.height as f64),
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
    }
}

/// Render a pixel point in an image pixel set
/// 
/// For each pixel offset (basically each new separate pixel)
/// check bounds - to render only a specific area of Mandelbrot set
/// and project a pixel from mandelbrot to real image
/// 
pub fn render(
        pixels: &mut [u8],
        bounds: SquareBoundsPx,
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.width * bounds.height,
        "Calculated bounds doesn't match the size of a pixels set");
    
    // Pixel counter to plot and image row by row
    let mut pixel_coord = Pixel { x: 0, y: 0 };
    let mut point;
    
    for (pindex, pixel) in pixels.iter_mut().enumerate() {
        pixel_coord.x = pindex / bounds.width;
        pixel_coord.y = pindex - (pixel_coord.x * bounds.width);
        // calculation is supposed to be faster than `if` jumpings
        // resets `y` (starts from 0) when `x` ticks
        // x: 0, y: 0..max; x:1, y: 0..max; x: 2, y: 0..max , and so on

        point = pixel_to_point(bounds, pixel_coord, upper_left, lower_right);
        *pixel = match mandelbrot::escape_time(point, 255) {
            None => 0,
            Some(count) => 255 - count as u8,
        };
    }
}

/// Writes pixels from set ina a Grayscale(8) image within given size bounds
pub fn write_image(
        filename: &str,
        pixels: &[u8],
        bounds: &SquareBoundsPx
) -> Result<(), Box<dyn std::error::Error>>
{
    let storage = File::create(filename)?;
    let (w, h) = (bounds.width as u32, bounds.height as u32);
    PngEncoder::new(storage).write_image(pixels, w, h, ColorType::L8)?;
    Ok(())
}



#[cfg(test)]
mod test_gui {
    use num::complex::Complex;
    use super::gui;

    #[test]
    fn test_pixel_to_point() {
        assert_eq!(
            gui::pixel_to_point(
                gui::SquareBoundsPx { width: 100, height: 200 },
                gui::Pixel { x: 25, y: 175 },
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 }),
            Complex {
                re: -0.5,
                im: -0.75,
            }
        );

    }
}