
use QuadTree;
use CheckPartition;

pub fn PartitionImage(
    image : ImageResult<DynamicImage>,
    reference_image : ImageResult<DynamicImage>,
    tree : QuadTree
) -> QuadTree{ 

    // run through from the middle
    if (CheckPartition::checkPartition(image, tree.x_start, tree.x_end, tree.y_start, tree.y_end)) {
        // do partition
    } else {
        // stop partition and do pattern matching and the result from the pattern matching will be summed for 3 other panel, then count the average of it (need to be between 0 and 1)
    }
}