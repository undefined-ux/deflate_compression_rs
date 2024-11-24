mod file_descriptor;
mod local_file_header;
use std::fs::Metadata;
use std::path::{Path, PathBuf};

use crate::compress_algorithm::Compressable;
use crate::zip::files::file_descriptor::FileDescriptor;
use crate::zip::files::local_file_header::LocalFileHeader;
use crate::utils::Crc32;

pub struct File {
    local_file_header: LocalFileHeader,
    file_data: Vec<u8>,
    file_descriptor: FileDescriptor,
    compression_algorithm: Box<dyn Compressable>,
    file_path: Box<PathBuf>,
}

impl File {
    pub fn new(
        file_path: &str,
        target_file_path: &str,
        compress_algo: impl Compressable + 'static,
    ) -> Result<Self, String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err("File does not exist".to_string());
        }
        if !path.is_file() {
            return Err(format!("{} is not a file", file_path));
        }

        let file_metadata: Metadata = match path.metadata() {
            Ok(metadata) => metadata,
            Err(e) => {
                panic!("Error: Failed to get file {} metadata: {}", file_path, e);
            }
        };

        let compress_result  = match compress_algo.compress_data(&path) {
            Ok(res) => res,
            Err(_) => panic!("Error: Failed to compress file {}", file_path),
        };

        let crc32 = match Crc32::calculate_file(path) {
            Ok(val) => val,
            Err(_) => panic!("Failed to calculate crc32."),
        };

        Ok(File {
            local_file_header: LocalFileHeader::new(
                &file_metadata,
                target_file_path,
                compress_algo.compress_method(),
            ),
            file_data: compress_result.data,
            file_descriptor: FileDescriptor::new(crc32, compress_result.compressed_size, file_metadata.len() as u64),
            compression_algorithm: Box::new(compress_algo),
            file_path: Box::new(path.to_path_buf()),
        })
    }

    pub fn get_local_file_header(&self) -> Vec<u8> {
        self.local_file_header.clone().into()
    }

    fn get_file_descriptor(&self) -> Vec<u8> {
        self.file_descriptor.clone().into()
    }

    fn get_file_data(&self) -> Vec<u8> {
        self.file_data.clone()
    }
}

impl Into<Vec<u8>> for File {
    fn into(self) -> Vec<u8> {
        let mut file = Vec::new();
        file.extend::<Vec<u8>>(self.local_file_header.into());
        file.extend::<Vec<u8>>(self.file_data.into());
        file.extend::<Vec<u8>>(self.file_descriptor.into());
        file
    }
}
