

/**
 * algorithm
 * 1. brute force baby
*/

use crate::{algorithm::rgbToGrayscale::rgbToGrayscale, structs::quadtree::Quadtree};
use image::DynamicImage;

pub fn bruteForceMatch(
    current_image : DynamicImage,
    current_root : Quadtree,
    reference_image : DynamicImage,
    reference_root : Quadtree,
) -> f64 {

    let mut sameCount = 0;
    let minWidth: u32 = f64::min((current_root.x_end - current_root.x_start) as f64, (reference_root.x_end - reference_root.x_start) as f64) as u32;
    let minHeigth : u32 = f64::min((current_root.y_end - current_root.y_start) as f64, (reference_root.y_end - reference_root.y_start) as f64) as u32;

    // compare tree and reference_tree which one has smaller width and smaller height
    for row in 0..minHeigth {
        for col in 0..minWidth {
            if rgbToGrayscale(&current_image, current_root.y_start + row, current_root.x_start + col) == 
            rgbToGrayscale(&reference_image, reference_root.y_start + row, reference_root.x_start + col) {
                sameCount += 1;
            }
        }
    }

    return sameCount as f64 / (minWidth * minHeigth) as f64;
}
