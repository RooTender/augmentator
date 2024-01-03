use std::io;
use std::path::{Path, PathBuf};
use std::error::Error;

fn get_image_paths(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>>
{
    let mut full_paths: Vec<PathBuf> = vec![];

    path.read_dir()?.filter_map(|e| e.ok()).for_each(|file|
    {
        let filename = file.file_name().to_string_lossy().into_owned();
        if let Some(extension) = filename.split('.').last()
        {
            if extension == "jpg" || extension == "png"
            {
                full_paths.push(file.path().canonicalize().unwrap());
            }
        }
    });

    Ok(full_paths)
}

fn main()
{
    println!("Please enter the directory containing the images: ");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);

    let paths = get_image_paths(Path::new(buffer.trim()))
        .expect("Failed to gather image paths");

    for path in paths {
        println!("{}", path.display());
    }
}
