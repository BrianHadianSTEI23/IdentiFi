
use image::GenericImageView;

pub fn QuadTreeMeanAbsoluteDeviation(
    image : ImageResult<DynamicImage>,
    x_start : i32,
    x_end : i32,
    y_start : i32,
    y_end : i32
) -> f64{

    // variables
    let total_pixels = (x_end - x_start + 1) * (y_end - y_start + 1);

    // calculate average pixel of red
    let mut sum_red = 0;

    for row in x_start..x_end + 1{
        for col in y_start..y_end + 1{
            sum_red += image.get_pixel(row, col).0[0];
        }
    }

    let mean_red = sum_red / total_pixels;

    // calculate average pixel of green
    let mut sum_green = 0;

    for row in x_start..x_end + 1{
        for col in y_start..y_end + 1{
            sum_green += image.get_pixel(row, col).0[1];
        }
    }

    let mean_green = sum_green / total_pixels;

    // calculate average pixel of red
    let mut sum_blue = 0;

    for row in x_start..x_end + 1{
        for col in y_start..y_end + 1{
            sum_blue += image.get_pixel(row, col).0[2];
        }
    }

    let mean_blue = sum_blue / total_pixels;

    return (mean_red + mean_green + mean_blue) / 3;
}