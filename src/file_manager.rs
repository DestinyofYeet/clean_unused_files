use std::fs::{File, Metadata};
use std::io::{Read, Write};
use std::os::linux::fs::MetadataExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataFile {
    pub data: Vec<String>
}

impl DataFile {
    pub fn new() -> DataFile {
        DataFile {
            data: Vec::new()
        }
    }
}

fn can_delete_file(metadata: Metadata) -> bool {
    return metadata.st_nlink() == 1;
}

pub fn del_file(path: &str) {
    std::fs::remove_file(path).unwrap();
    println!("Deleted file {}", path);
}

pub fn get_size(path: &str) -> u64 {
    std::fs::metadata(path).unwrap().len()
}

pub fn format_bytes(mut bytes: u64) -> String {
    let formats = ["B", "KB", "MB", "GB", "TB", "PB"];


    let mut counter = 0;
    while bytes > 1000 {
        if counter == (formats.len() - 1) {
            break;
        }

        bytes = bytes / 1000;

        counter += 1;
    }

    let string = format!("{} {}", bytes.to_string(), formats[counter]);

    return string;
}

pub fn handle_path(file_path: &str) -> bool {

    if file_path.starts_with("/mnt/data/data/media/downloads/qbit/permanent") {
        return false;
    }

    let metadata = std::fs::metadata(&file_path);

        if metadata.is_err() {
            eprintln!("Failed to stat {}!", file_path);
            return false;
        }

        let unwrapped_metadata = metadata.unwrap();

        if unwrapped_metadata.is_dir(){
            let paths = std::fs::read_dir(&file_path).unwrap();

            let mut can_delete_dir: bool = false;

            for path in paths {
                if can_delete_file(path.unwrap().metadata().unwrap()) {
                    if can_delete_dir == false {
                        can_delete_dir = true;
                    }
                }
            }

            if can_delete_dir {
                // println!("Can delete directory: {}", file_path);
                return true;
            }

        } else {
            if can_delete_file(unwrapped_metadata) {
                // println!("Can delete file: {}", file_path);
                return true;
            }
        }

    return false;
}

pub fn does_file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn write_json_file(path: &str, content: &DataFile) {
    let mut file = std::fs::OpenOptions::new().create(true).write(true).open(path).unwrap();

    let content_to_be_written = serde_json::to_string_pretty(content).unwrap();

    match file.write(content_to_be_written.as_bytes()) {
        Err(e) => panic!("Could not write to file {}, because {}", path, e),
        Ok(_) => println!("Wrote to file {}, {}", path, content_to_be_written)
    }

    file.flush().unwrap();

}

pub fn read_file_json(path: &str) -> DataFile {
    let mut file = match File::open(&path) {
        Err(e) => panic!("Failed to open file {}, because: {}", path, e),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => panic!("Could not read content of {} because {}", path, e),
        Ok(_) => {
            println!("File read is: {}", s);
            let value: DataFile = serde_json::from_str(s.as_str()).unwrap();
            file.flush().unwrap();

            return value;
        }
    }
}
