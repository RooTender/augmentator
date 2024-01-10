mod augment;
mod utils;

use augment::augment_dataset;
use utils::{preprocess_data, DimensionFilter, delete_unpaired_files};
use std::path::Path;

fn main() {
    // let input_dir = Path::new("/home/rootender/Documents/VanillaDefault-Resource-Pack-16x-1.20/assets");
    // let output_dir = Path::new("out");
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

    delete_unpaired_files(dir_1, dir_2, 2).expect("Failure");

    let dir = Path::new("out");
    augment_dataset(dir_1, dir, 123).expect("Failed to augment");
}
