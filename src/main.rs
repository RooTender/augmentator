mod augment;
mod utils;

use augment::augment_dataset;
use utils::{convert_images, delete_unpaired_files, preprocess_data, ConversionFormat, DimensionFilter};
use std::path::Path;

fn main() {
    // let input_dir = Path::new("/home/rootender/Documents/Faithful 32x - 1.20.4/assets");
    // let output_dir = Path::new("faith");
    // let min_count = 5;

    // let filters: Vec<DimensionFilter> = vec![
    //     Box::new(move |_, files| files.len() >= min_count),
    //     Box::new(move |dim, _| dim.0 == dim.1)
    //     // Add more filters as needed
    // ];

    // if let Err(e) = preprocess_data(input_dir, output_dir, filters) {
    //     eprintln!("Error during preprocessing: {}", e);
    // }

    let dir_1 = Path::new("original");
    let dir_2 = Path::new("faith");

    // delete_unpaired_files(dir_1, dir_2, 2)
    //     .expect("Failed to remove unpaired images");

    // convert_images(dir_1, ConversionFormat::RGBA)
    //     .expect("Failed to convert images");
    // convert_images(dir_2, ConversionFormat::RGBA)
    //     .expect("Failed to convert images");

    
    let dir = Path::new("aug_original");
    augment_dataset(dir_1, dir, 123).expect("Failed to augment");

    // let dir = Path::new("aug_faith");
    // augment_dataset(dir_2, dir, 123).expect("Failed to augment");
}
