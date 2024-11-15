use anyhow::Result;
use glob::glob;

pub fn find_files(glob_pattern: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in glob(glob_pattern)? {
        if let Ok(path) = entry {
            if path.is_file() {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }

    Ok(files)
}
