use clap::Parser;
use console::{style, Term};
use files::find_files;
use helpers::{append_to_path, write_to_output};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{from_str, Map, Value};
use std::{
    collections::HashMap, env, fs, hash::Hash, path::PathBuf, process::exit, sync::Mutex, time::Instant
};
use watch::watch;

use crate::file_map::{FileMap};

pub mod file_map;
pub mod files;
pub mod helpers;
pub mod watch;

#[derive(Debug, Clone)]
struct DuplicateFileError {
    component: String,
    file_paths: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Watch for file changes and merge them automatically
    #[arg(short, long, default_value = "false")]
    watch: bool,

    /// Output file
    #[clap(long, short, value_parser = clap::value_parser!(PathBuf), default_value="output.json")]
    output: PathBuf,

    /// Input directory
    #[arg(short, long, value_parser = clap::value_parser!(PathBuf), default_value = ".")]
    input_dir: PathBuf,
}

lazy_static! {
    static ref FILENAME_REGEX: Regex = Regex::new(r#"([^\.]+)\.labels\.json$"#).unwrap();

    // There can only be one label file per component, holding these references to make sure there aren't duplicates
    static ref GLOBAL_FILE_MAP: Mutex<HashMap<String, PathBuf>> = Mutex::new(HashMap::new());

    // Global file map to store all the file contents
    // TODO: Rename to GLOBAL_FILE_MAP once replaced
    static ref GLOBAL: Mutex<HashMap<String,FileMap>> = Mutex::new(HashMap::new());
}

pub fn main() {
    let start = Instant::now();
    // Set up logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let term = Term::stdout();
    // Parse arguments
    let args = Args::parse();

    // Check if the input directory exists, if not, use the current directory
    // We might just throw an error as it already defaults back to current directory
    let path = if args.input_dir.exists() && args.input_dir.is_dir() {
        args.input_dir
    } else {
        log::error!("Input directory {} does not exist", args.input_dir.display());
        exit(1)
    };

    let output_path = if args.output.parent().is_some() {
        args.output
    } else {
        append_to_path(&path, args.output.to_str().unwrap())
    };

    let files = find_files(&path, &FILENAME_REGEX);

    let mut merged_data: Map<String, Value> = Map::new();



    if let Err(e) = create_initial_map(files) {
        let error_line = format!(
            "❌ Duplicate file found for: {}, [{}]",
            e.component,
            e.file_paths.join(", ")
        );
        term.write_line(&format!("{}", style(error_line).red()))
            .unwrap_or(());
        exit(1);
    }

    for (key, value) in GLOBAL.lock().unwrap().iter() {
        merged_data.insert(key.clone(), value.contents.clone());
    }


    // Write the merged data to the output file
    if let Err(er) = write_to_output(&mut merged_data, &output_path) {
        log::error!("An error occurred while writing to output file: {}", er);
        exit(1);
    }

    let duration = start.elapsed();
    log::info!("Time elapsed: {:?}", duration);

    // Initial merge has been done, check if application should keep running in watch mode
    if args.watch {
        term.write_line(&format!("{}", style("Starting in watch mode").yellow()))
            .unwrap_or(());

        // Start watching for file changes, see watch.rs for implementation
        if let Err(error) = watch(&path) {
            log::error!(
                "An error occurred while watching for file changes: {}",
                error
            );
        }
    }
}

/// Create initial map that will be used to merge data from files
/// It will also check for duplicate files for the same component and return an error when that happens
fn create_initial_map(files: Vec<String>) -> Result<(), DuplicateFileError>{
    let mut map = GLOBAL.lock().unwrap();
    for file in files {
        let contents = fs::read_to_string(&file).expect("Unable to read file");
        let data: Value = from_str(&contents).expect("Unable to parse JSON");
        let file_name = file.split('/').last().unwrap_or("");
        let name = FILENAME_REGEX
            .captures(file_name)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        if name.is_empty() {
            log::warn!(
                "File name does not match the expected pattern: {}",
                file_name
            );
            continue;
        }

        // We don't allow multiple files to merge to the same key, show an error when this initially happens
        if map.contains_key(name) {
            let current_file = map.get(name).unwrap().file_path.to_str().unwrap();

            return Err(DuplicateFileError {
                component: String::from(name),
                file_paths: vec![file.clone(), String::from(current_file)],
            });
        };

        map.insert(name.to_string(), FileMap { name: name.to_string(), file_path: PathBuf::from(file.clone()), contents: data.clone()});
    }
    Ok(())
}
