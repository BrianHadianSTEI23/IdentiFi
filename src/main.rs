/*
algorithm
1. get the middle of the fingerprint first
2. do divide and conquer until the backgronud is more black than white
3. if it's done, then do pattern matching
4. for each pixel checked, do mean absolute deviation for counting its error
5. for each error in the 4 panel, determine its average then sum all the error in the 4 panel then the average before is divided by the sum
6. the value from (5) will then be the error of the four panel counted
*/

use std::io;
use image::GenericImageView;
mod structs;

fn main() {
    // input the file

    let mut file_target : String = String::new();
    println!("Insert your file to be checked: ");
    io::stdin().read_line(&mut file_target).expect("Invalid file");
    let image = image::open("../test/in/DB1/101_1.tif").expect("Failed to open image"); // "../test/in/DB1/101_1.tif"
    println!("{:?}", image.get_pixel(194, 187).0);
    
    // preprocess the data of pixel in the file to be of quadtree
    let mut root = structs::quadtree::Quadtree::new(0, 0, 0, 0);

    // partition all (always start from the middle)

}
