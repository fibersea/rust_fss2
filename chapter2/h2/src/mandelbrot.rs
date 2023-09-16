/*! Contains functionality for a Mandelbrot math calculations
 * 
 */

use num::complex::Complex;


/// Detects if a given point `c` is within the mandelbrot set
/// 
/// If `z` crosses the circle of radius `2`
/// then the iteration count is returned.
/// Otherwise if `z` for a given `c` varying around emphasized boundary
/// None is returned and `z` is considered to be a member
/// of the mandelbrot set.
/// 
/// # Arguments
/// 
/// * `c` - is the constant
/// * `limit` - is the recurrent iteration count limit
/// 
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for iteration_count in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(iteration_count);
        }
        z = z * z + c;
    }

    None
}
