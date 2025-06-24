use crate::{algorithm::rgbToGrayscale::rgbToGrayscale, structs::quadtree::Quadtree};
use image::{DynamicImage};

pub fn PreprocessImage(
    current_image: &DynamicImage,
) -> Quadtree {
    let width = current_image.width();
    let height = current_image.height();

    let mut root = Quadtree::new(0, 0, 0, 0);
    let mut found = false;

    // Find top-left corner of fingerprint
    'outer: for col in 0..width {
        for row in 0..col {
            if rgbToGrayscale(current_image, row, col) < 245 {
                root.x_start = row;
                root.y_start = 0;
                found = true;
                break 'outer;
            }
        }
    }

    if !found {
        return root; // Return empty if no fingerprint found
    }

    // Find bottom-right corner
    for row in (0..height).rev() {
        for col in (0..width).rev() {
            if rgbToGrayscale(current_image, row, col) < 245 {
                root.x_end = row;
                root.y_end = current_image.height() - 1;
                return root;
            }
        }
    }

    root
}
