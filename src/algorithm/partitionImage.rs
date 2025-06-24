
use image::DynamicImage;
use crate::algorithm::knuthMorrisPrattMatch::KnuthMorrisPrattMatch;
use crate::structs::quadtree::Quadtree;
use crate::algorithm::checkPartition::CheckPartition;

pub fn PartitionImage(
    current_image : &DynamicImage,
    current_root : &Quadtree,
    reference_image :&DynamicImage,
    reference_root : &Quadtree,
) -> f64{ 

    // run through from the middle
    if (CheckPartition(current_image, &current_root, reference_image, &reference_root)) {

        // do partition for current
        let current_width : u32 = current_root.x_end - current_root.x_start;
        let current_height : u32 = current_root.y_end - current_root.y_start;
        
        let current_left_width_partition:u32 = current_width / 2;

        let current_top_height_partition: u32 = current_height / 2;
        
        let mut current_top_left:Quadtree = Quadtree::new(current_root.x_start, 
                                                            current_root.x_start + current_left_width_partition,
                                                            current_root.y_start, 
                                                            current_root.y_start + current_top_height_partition);
        let mut current_top_right: Quadtree =Quadtree::new(current_root.x_start + current_left_width_partition + 1, 
                                                            current_root.x_end,
                                                        current_root.y_start, 
                                                            current_root.y_start + current_top_height_partition);
        let mut current_bottom_right: Quadtree =Quadtree::new(current_root.x_start + current_left_width_partition + 1, 
                                                                current_root.x_end,
                                                            current_root.y_start + current_top_height_partition + 1, 
                                                            current_root.y_end);
        let mut current_bottom_left: Quadtree =Quadtree::new(current_root.x_start, 
                                                                current_root.x_start + current_left_width_partition,
                                                            current_root.y_start + current_top_height_partition + 1, 
                                                            current_root.y_end);
    
       // do partition for reference
        let reference_width : u32 = reference_root.x_end - reference_root.x_start;
        let reference_height : u32 = reference_root.y_end - reference_root.y_start;
        
        let reference_left_width_partition:u32 = reference_width / 2;
        let reference_right_width_partition: u32 = reference_width - reference_left_width_partition;

        let reference_top_height_partition: u32 = reference_height / 2;
        let reference_bottom_height_partition: u32 = reference_height - reference_top_height_partition;
        
        let mut reference_top_left:Quadtree = Quadtree::new(reference_root.x_start, 
                                                            reference_root.x_start + reference_left_width_partition,
                                                            reference_root.y_start, 
                                                            reference_root.y_start + reference_top_height_partition);
        let mut reference_top_right: Quadtree =Quadtree::new(reference_root.x_start + reference_right_width_partition + 1, 
                                                            reference_root.x_end,
                                                        reference_root.y_start, 
                                                            reference_root.y_start + reference_top_height_partition);
        let mut reference_bottom_right: Quadtree =Quadtree::new(reference_root.x_start + reference_right_width_partition + 1, 
                                                                reference_root.x_end,
                                                            reference_root.y_start + reference_top_height_partition + 1, 
                                                            reference_root.y_end);
        let mut reference_bottom_left: Quadtree =Quadtree::new(reference_root.x_start, 
                                                                reference_root.x_start + reference_left_width_partition,
                                                            reference_root.y_start + reference_top_height_partition + 1, 
                                                            reference_root.y_end);

        // repeat for each current and reference pair
        let sum = PartitionImage(current_image, &current_top_left, reference_image, &reference_top_left) +
          PartitionImage(current_image, &current_top_right, reference_image, &reference_top_right) +
          PartitionImage(current_image, &current_bottom_left, reference_image, &reference_bottom_left) +
          PartitionImage(current_image, &current_bottom_right, reference_image, &reference_bottom_right);

        return sum / 4.0;


    } else {
        // stop partition and do pattern matching and the result from the pattern matching will be summed for 3 other panel, then count the average of it (need to be between 0 and 1)
        return KnuthMorrisPrattMatch(current_image, &current_root, reference_image, reference_root);
    }
}