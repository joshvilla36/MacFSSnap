use std::fs;
use std::path::PathBuf;
use clap::value_parser;
use clap::{Command, Arg};
use chrono::{DateTime, Utc};
use sha256;

// struct FileRecord{
//     file_path: String,
//     file_name: String,
//     ftype: char,
//     size: u64,
//     created: String,
//     modified: String,
//     accessed: String,
//     sha256: String
// }

fn list_files(path_to_search: &PathBuf) {

    match fs::read_dir(path_to_search) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(meta) = entry.metadata() {
                        let path = entry.path();
                        let name = entry.file_name();
                        let size = meta.len();
                        let created: DateTime<Utc> = meta.created().unwrap().into();
                        let modified: DateTime<Utc> = meta.modified().unwrap().into();
                        let accessed: DateTime<Utc> = meta.accessed().unwrap().into();
                        let ftype = if meta.is_dir(){
                            "d"
                        } else if meta.is_file(){
                            "f"
                        } else{
                            "l"
                        };
                        let hash = if !(meta.is_dir()) && !(meta.is_symlink()){
                            calc_sha256(entry.path())
                        } else{
                            "".to_string()
                        };
                        println!("{},{},{},{},{},{},{},{}", path.into_os_string().to_string_lossy(), name.to_string_lossy(), ftype, size, created, modified, accessed, hash);
                        if meta.is_dir(){
                            list_files(&entry.path());
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn calc_sha256(file_path: PathBuf) -> String {
    let bytes = std::fs::read(file_path).unwrap();
    return sha256::digest(&bytes);
}

fn main() {
    let matches = Command::new("MacFSSnap")
    .version("1.0")
    .author("Josh Villanueva")
    .about("Description of your program")
    .arg(
        Arg::new("path")
            .short('p')
            .long("path")
            .value_name("FILE_PATH")
            .help("Sets the directory to list")
            .value_parser(value_parser!(PathBuf))
            .default_value("/"),
    )
    .get_matches();

    if let Some(file_path) = matches.get_one::<PathBuf>("path") {
        println!("file_path,file_name,ftype,size,created,modified,accessed,sha256");
        list_files(file_path);
    }
    
}