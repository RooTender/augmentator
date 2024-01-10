mod utils;

use utils::{preprocess_data, DimensionFilter};
use std::path::Path;

fn main() {
    let input_dir = Path::new("/home/rootender/Documents/VanillaDefault-Resource-Pack-16x-1.20/assets");
    let output_dir = Path::new("out");
    let min_count = 5;

    let filters: Vec<DimensionFilter> = vec![
        Box::new(move |_, files| files.len() >= min_count),
        Box::new(move |dim, _| dim.0 == dim.1)
        // Add more filters as needed
    ];

    if let Err(e) = preprocess_data(input_dir, output_dir, filters) {
        eprintln!("Error during preprocessing: {}", e);
    }
}
