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
    let paths = get_image_paths(Path::new("src/test"))
        .expect("Failed to collect image paths");

    for path in paths {
        println!("{}", path.display());
    }
}
