use std::path::{Path, PathBuf};
use std::error::Error;

fn get_image_paths(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>>
{
    let mut full_paths: Vec<PathBuf> = vec![];

    for file in path.read_dir()?.filter_map(|e| e.ok())
    {
        let filename = file.file_name().to_string_lossy().into_owned();
        if let Some(extension) = filename.split('.').last()
        {
            if extension == "jpg" || extension == "png"
            {
                full_paths.push(file.path().canonicalize().unwrap());
            }
        }
    }

    Ok(full_paths)
}

fn main()
{

}
