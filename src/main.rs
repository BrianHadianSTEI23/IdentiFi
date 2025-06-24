/*
algorithm
1. get the middle of the fingerprint first
2. do divide and conquer until the backgronud is more white than blakck
3. if it's done, then do pattern matching
4. for each pixel checked, do mean absolute deviation for counting its error
5. for each error in the 4 panel, determine its average then sum all the error in the 4 panel then the average before is divided by the sum
6. the value from (5) will then be the error of the four panel counted
*/

use std::fs;
use std::io::{self, Write};
use crate::algorithm::partitionImage::PartitionImage;

mod structs;
mod algorithm;

fn main() {
    // Ask how many datasets to show
    println!("How many dataset files do you want to display?");
    let mut num_display = String::new();
    io::stdin().read_line(&mut num_display).expect("Failed to read input");
    let num_display: usize = num_display.trim().parse().expect("Please enter a valid number");

    // Load all files in test/DB1/
    let paths = fs::read_dir("test/DB1").expect("Failed to read directory");
    let mut files: Vec<String> = paths
        .filter_map(Result::ok)
        .map(|entry| entry.path().display().to_string())
        .filter(|name| name.ends_with(".tif"))
        .collect();

    files.sort(); // optional: sort alphabetically

    // Show only the requested number
    let display_count = std::cmp::min(num_display, files.len());
    println!("\nAvailable files:");
    for (i, file) in files.iter().take(display_count).enumerate() {
        println!("{}: {}", i + 1, file);
    }

    // Ask user to select file index
    println!("\nEnter the number of the image to be checked:");
    let mut selected_index = String::new();
    io::stdin().read_line(&mut selected_index).expect("Failed to read index");
    let selected_index: usize = selected_index.trim().parse().expect("Invalid number");

    if selected_index == 0 || selected_index > display_count {
        println!("Invalid selection");
        return;
    }

    let selected_file = &files[selected_index - 1];

    println!("You selected: {}", selected_file);

    let current_image = image::open(selected_file).expect("Failed to open selected image");
    let reference_image = image::open("test/DB1/101_2.tif").expect("Failed to open reference image");

    // preprocess
    let current_root = algorithm::preprocessImage::PreprocessImage(&current_image);
    let reference_root = algorithm::preprocessImage::PreprocessImage(&reference_image);

    let similarity = PartitionImage(&current_image, current_root, &reference_image, reference_root);

    println!("\nSimilarity: {}", similarity);
}

