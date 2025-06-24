
use image::DynamicImage;

use crate::{algorithm::rgbToGrayscale};

pub fn QuadTreeDeviation(
    image : &DynamicImage,
    width : u32,
    height : u32
) -> f64{

    // variables
    let total_pixels = width * height;

    // calculate average pixel of red
    let mut sum_grayscale: u32 = 0;

    for row in 0..height{
        for col in 0..width{
            sum_grayscale += rgbToGrayscale::rgbToGrayscale(image, row, col);
        }
    }

    return (sum_grayscale / total_pixels) as f64;
}