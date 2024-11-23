use std::path::Path;
use std::fs::{self, Metadata};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::compress_algorithm::Compressable;

pub struct LocalFileHeader {
    version: [u8; 4],
    flags: [u8; 2],
    compression_method: [u8; 2],
    last_modification_time: [u8; 2],
    last_modification_date: [u8; 2],
    crc32: [u8; 4],
    compressed_size: [u8; 4],   // zip64格式中被设定为0xffffffff
    uncompressed_size: [u8; 4], // zip64格式中被设定为0xffffffff
    file_name_length: [u8; 2], 
    extra_field_length: [u8; 2],
    file_name: Vec<u8>,
    extra_field: Vec<u8>
}


impl LocalFileHeader {
    pub fn new(file: &Path, target_file_name: &str, compression_method: impl Compressable) -> () {
        let file_name: Vec<u8> = target_file_name.as_bytes().to_vec();
        let version: [u8; 2] = [0x45, 0x00];
        let file_name_length = file_name.len() as u16;
        let extra_field: Vec<u8> = Vec::new();
        let mut last_modification_time: [u8; 2];
        let mut last_modification_date: [u8; 2];
        let file_metadata: Option<Metadata> = match file.metadata() {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                eprintln!("Error: Failed to get file {} metadata: {}", file.to_str().unwrap(), e);
                None
            }
        };
        let mut last_modification_timestamp: u64;

        if let Some(metadata) = file_metadata {
            last_modification_timestamp = match metadata.modified() {
                Ok(time) => time.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                Err(e) => {
                    eprintln!("Error: Failed to get file {} last modification datetime: {}", file.to_str().unwrap(), e);
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() // 使用当前时间作为文件最后修改时间的时间戳
                },
            };
        }else {
            last_modification_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }
        
    }
}