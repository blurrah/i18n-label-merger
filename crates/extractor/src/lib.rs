use std::path::PathBuf;

pub fn find_typescript_files(path: &PathBuf, regex: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    todo!("Implement finding files")
}

pub fn find_translation_namespace() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    todo!("Implement finding translation namespaces")
}

pub fn create(output_file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    todo!("Implement creating output json file")
}

