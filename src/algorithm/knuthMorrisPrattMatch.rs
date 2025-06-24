
use std::collections::HashMap;
use image::{DynamicImage};
use crate::{algorithm::rgbToGrayscale::rgbToGrayscale, structs::quadtree::Quadtree};

/// Get KMP border table for a single row starting from x_start
pub fn getBorderTable(
    reference_image: &DynamicImage,
    x_start: u32,
    row: u32,
    pattern_length: u32,
) -> HashMap<u32, u32> {
    let mut border_table = HashMap::new();
    border_table.insert(0, 0);

    let mut iter_for_prefix = 0;
    let mut iter_for_suffix = 1;

    while iter_for_suffix < pattern_length {
        let gray_suffix = rgbToGrayscale(reference_image, row, x_start + iter_for_suffix);
        let gray_prefix = rgbToGrayscale(reference_image, row, x_start + iter_for_prefix);

        if gray_suffix == gray_prefix {
            iter_for_prefix += 1;
            border_table.insert(iter_for_suffix, iter_for_prefix);
            iter_for_suffix += 1;
        } else if iter_for_prefix > 0 {
            iter_for_prefix = *border_table.get(&(iter_for_prefix - 1)).unwrap_or(&0);
        } else {
            border_table.insert(iter_for_suffix, 0);
            iter_for_suffix += 1;
        }
    }

    border_table
}

pub fn KnuthMorrisPrattMatch(
    current_image: &DynamicImage,
    current_root: &Quadtree,
    reference_image: &DynamicImage,
    reference_root: &Quadtree,
) -> f64 {
    let mut same_count: u32 = 0;
    let pattern_length = reference_root.x_end - reference_root.x_start;
    let mut total_pixels: u32 = 0;

    for row in current_root.y_start..current_root.y_end {
        let border_table = getBorderTable(
            reference_image,
            reference_root.x_start,
            row,
            pattern_length,
        );

        let mut iter_for_prefix = 0;
        let mut iter_for_suffix = 0;

        while current_root.x_start + iter_for_suffix < current_root.x_end {
            let gray_current = rgbToGrayscale(
                current_image,
                row,
                current_root.x_start + iter_for_suffix,
            );
            let gray_reference = rgbToGrayscale(
                reference_image,
                row,
                reference_root.x_start + iter_for_prefix,
            );

            if gray_current == gray_reference {
                if iter_for_prefix == pattern_length - 1 {
                    same_count += 1;
                    iter_for_prefix = 0;
                    iter_for_suffix += 1;
                } else {
                    iter_for_prefix += 1;
                    iter_for_suffix += 1;
                }
            } else if iter_for_prefix > 0 {
                iter_for_prefix = *border_table.get(&(iter_for_prefix - 1)).unwrap_or(&0);
            } else {
                iter_for_suffix += 1;
            }
            total_pixels += 1;
        }
    }

    same_count as f64 / total_pixels as f64
}
