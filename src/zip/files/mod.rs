mod local_file_header;

use std::path::{Path, PathBuf};

use crate::compress_algorithm::Compressable;

pub struct File {
    pub local_file_header: Vec<u8>,
    pub file_data: Vec<u8>,
    pub file_descriptor: Option<Vec<u8>>,
    compression_algorithm: Box<dyn Compressable>,
    file_path: Box<PathBuf>,
    target_file_path: String
}


impl File {
    pub fn new(file_path: &str, target_file_path: &str, compress_algo: impl Compressable + 'static) -> Result<Self, String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err("File does not exist".to_string());
        }
        if !path.is_file() {
            return Err(format!("{} is not a file", file_path));
        }

        Ok(File {
            local_file_header: Vec::new(),
            file_data: Vec::new(),
            file_descriptor: None,
            compression_algorithm: Box::new(compress_algo),
            file_path: Box::new(path.to_path_buf()),
            target_file_path: target_file_path.to_string()
        })
    }

    fn get_local_file_header(&self) -> Vec<u8> {
        let local_file_header = Vec::new();
        local_file_header
    }


    fn get_file_descriptor(&self) -> Vec<u8> {
        let file_descriptor = Vec::new();
        file_descriptor
    }

    fn get_file_data(&self) -> Result<Vec<u8>, ()> {
        self.compression_algorithm.compress_data(&self.file_path)
    }
}



impl Into<Vec<u8>> for File {
    fn into(self) -> Vec<u8> {
        let mut file = Vec::new();
        file.extend(self.get_local_file_header());
        file.extend(self.get_file_data().unwrap());
        file.extend(self.get_file_descriptor());
        file
    }
}