extern crate num;
use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let _z = Complex { re: 0.0, im: 0.0 };

}

/// Try to determine if 'c' is in the Mandelbrot set, using at most 'limit' iterations to decide.
///
/// if 'c' is not a member, return 'Some(i)', where 'i' is the number of iterations it took for 'c'
/// to leave the circle of radius 2 centered on the origin. If 'c' seems to be a member
/// (more precisely, if we reached the iteration limit without being able to prove that 'c' is not
/// a member), return None
#[allow(dead_code)]
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
#[allow(dead_code)]
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match(T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

/// Parse a pair of floating-point numbers separated by a comma as a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex{re, im})
    }
}

/// Given the row and column of a pixel in the output image, return the corresponding point
/// on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels
/// `pixel` is a (column, row) pair indicating a particular pixel in that image
/// The `upper_left` and `lower_right` parameters are points on the complex plane designating
/// the area our image covers
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>) -> Complex<f64> {

    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im
    );

    // Why substraction here? pixel.1 increases as we go down,
    // but the imaginary component increases as we go up
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

/// Render a rectangle of the Manderlbrot set into a buffer of pixels
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-left and
/// lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>, lower_right: Complex<f64>) {

    assert_eq!(pixels.len(), bounds.0 * bounds.1);
    for row in 0..bounds.0 {
        for column in 0..bounds.1 {
            let point = pixel_to_point(bounds, (row, column), upper_left, lower_right);
            pixels[row * bounds.0 + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
    assert_eq!(parse_pair("10x20", 'x'), Some((10, 20)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex(""), None);
    assert_eq!(parse_complex("1.25,2.25"), Some(Complex {re: 1.25, im: 2.25}));
    assert_eq!(parse_complex(",2.25"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point(
        (100, 100), (25, 75), Complex{re: -1.0, im: 1.0}, Complex{re: 1.0, im: -1.0}),
               Complex{re: -0.5, im: -0.5}
    );
    assert_eq!(pixel_to_point(
        (100, 100), (0,0), Complex{re:-1.0, im:1.0}, Complex{re:1.0, im:-1.0}),
               Complex{re:-1.0, im: 1.0}
    );
    assert_eq!(pixel_to_point(
        (100, 100), (100,100), Complex{re:-1.0, im:1.0}, Complex{re:1.0, im:-1.0}),
               Complex{re:1.0, im: -1.0}
    );
}

#[test]
fn test_render(){
    let pixels =
}