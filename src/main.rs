/*
algorithm
1. get the middle of the fingerprint first
2. do divide and conquer until the backgronud is more white than blakck
3. if it's done, then do pattern matching
4. for each pixel checked, do mean absolute deviation for counting its error
5. for each error in the 4 panel, determine its average then sum all the error in the 4 panel then the average before is divided by the sum
6. the value from (5) will then be the error of the four panel counted
*/
use std::{fs, string, time};
use std::io::{self, Write};
use crate::algorithm::partitionImage::PartitionImage;

mod structs;
mod algorithm;

fn main() {
    let mut avg_sum: f64 = 0.0;
    let mut count: u32 = 0;

    // List folders inside test/
    let test_base = "test";
    let folders: Vec<_> = fs::read_dir(test_base)
        .expect("Failed to read test/ directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .collect();

    if folders.is_empty() {
        println!("No folders found in test/");
        return;
    }

    println!("Available folders:");
    for (i, folder) in folders.iter().enumerate() {
        println!("{}: {}", i + 1, folder.file_name().to_string_lossy());
    }

    // Let user pick a folder
    println!("\nEnter the number of the folder you want to use:");
    let mut folder_index = String::new();
    io::stdin().read_line(&mut folder_index).expect("Failed to read input");
    let folder_index: usize = folder_index.trim().parse().expect("Please enter a valid number");

    if folder_index == 0 || folder_index > folders.len() {
        println!("Invalid folder selection");
        return;
    }

    let folder_name = folders[folder_index - 1].file_name().to_string_lossy().into_owned();
    let folder_path = format!("{}/{}", test_base, folder_name);

    // List .tif files inside that folder
    let mut files: Vec<_> = fs::read_dir(&folder_path)
        .expect("Failed to read folder")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext == "tif")
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect();

    files.sort();

    if files.len() < 2 {
        println!("Need at least 2 .tif files in the folder to compare.");
        return;
    }

    println!("\nAvailable images:");
    for (i, file) in files.iter().enumerate() {
        println!("{}: {}", i + 1, file.display());
    }

    //  Ask user to choose one file to compare against others
    println!("\nEnter the number of the image you want to check:");
    let mut selected_index = String::new();
    io::stdin().read_line(&mut selected_index).expect("Failed to read input");
    let selected_index: usize = selected_index.trim().parse().expect("Please enter a valid number");

    if selected_index == 0 || selected_index > files.len() {
        println!("Invalid file selection");
        return;
    }

    let selected_file = &files[selected_index - 1];
    println!("Selected image: {}", selected_file.display());

    // Load selected image
    let current_image = image::open(selected_file).expect("Failed to open selected image");
    let current_root = algorithm::preprocessImage::PreprocessImage(&current_image);
    let mut max_similarity: f64 = 0.0;
    let mut max_similarity_file: String = String::new();

    // Compare to all other files
    println!("\nComparing with other images in the folder...\n");
    let start = time::Instant::now();
    for (i, file) in files.iter().enumerate() {

        let reference_image = image::open(file).expect("Failed to open reference image");
        let reference_root = algorithm::preprocessImage::PreprocessImage(&reference_image);

        let similarity = PartitionImage(&current_image, &current_root, &reference_image, &reference_root);
        avg_sum += similarity;
        count += 1;
        if similarity > max_similarity {
            max_similarity = similarity;
            max_similarity_file = file.display().to_string();
        }
        println!("{} -> Similarity: {:.7}%", file.display(), similarity * 100.0);
    }
    let duration = start.elapsed();

    println!("\nAverage similarity : {}%", avg_sum / count as f64);
    println!("File with max similarity : {} with similarity {}%", max_similarity_file, max_similarity * 100.0);
    println!("Time elapsed : {:?} ms", duration.as_millis());
}

