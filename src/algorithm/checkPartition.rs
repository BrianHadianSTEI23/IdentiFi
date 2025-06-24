
// specification : determine if part after partition is still needed to be partitioned 
// by checking whether the deviation from current_root is larger or not than deviation 
// from reference_root

use image::{DynamicImage};
use crate::{algorithm::{quadTreeDeviation::{QuadTreeDeviation}}, structs::quadtree::Quadtree};

pub fn CheckPartition(
    current_image : &DynamicImage,
    current_root : &Quadtree,
    reference_image : &DynamicImage,
    reference_root : &Quadtree,
) -> bool {
    let factor : f64 = 1.0;

    let current_width : u32 = current_root.x_end - current_root.x_start;
    let current_height : u32 = current_root.y_end - current_root.y_start;
    let reference_width : u32 = reference_root.x_end - reference_root.x_start;
    let reference_height : u32 = reference_root.y_end - reference_root.y_start;
    
    // check which root is larger between reference_root 
    if current_width < reference_width {
        if current_height < reference_height {
            return QuadTreeDeviation(current_image, current_width, current_height) < (QuadTreeDeviation(reference_image, current_width, current_height) / factor);
        } else {
            return QuadTreeDeviation(current_image, current_width, reference_height) < (QuadTreeDeviation(reference_image, current_width, reference_height) / factor);
        }
    } else {
        if current_height < reference_height {
            return QuadTreeDeviation(current_image, reference_width, current_height) < (QuadTreeDeviation(reference_image, reference_width, current_height) / factor);
        } else {
            return QuadTreeDeviation(current_image, reference_width, reference_height) < (QuadTreeDeviation(reference_image, reference_width, reference_height) / factor);
        }
    }
}