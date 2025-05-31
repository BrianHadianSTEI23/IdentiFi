
// specification : determine if part after partition is still needed to be partitioned

use QuadTree;

pub fn CheckPartition(
    image : ImageResult<DynamicImage>,
    x_start : i32,
    x_end : i32,
    y_start : i32,
    y_end : i32
) -> bool {
    for row in x_start..x_end + 1 {
        for col in y_start..y_end + 1 {
            let grayscale_equivalent:i32 = (0.299 * image.get_pixel(row, col).0[0]) + (0.587 * image.get_pixel(row, col).0[0]) + (0.114 * image.get_pixel(row, col).0[0]).floor();
            if  grayscale_equivalent > 245{
                // keep doing partition
                return true;
            }
        }
    }
    return false;
}