

use image::{DynamicImage, GenericImageView};


pub fn rgbToGrayscale (input_image : &DynamicImage,
                        row : u32,
                        col : u32,) -> u32{
    return ((0.299 * (input_image.get_pixel(col, row).0[0] as f64)) 
    + (0.587 * (input_image.get_pixel(col, row).0[1] as f64)) 
    + (0.114 * (input_image.get_pixel(col, row).0[2] as f64))).floor() as u32;
}