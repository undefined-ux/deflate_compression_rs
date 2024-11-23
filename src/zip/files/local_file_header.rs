use std::path::Path;
use std::fs::Metadata;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::DateTime;

#[derive(Debug, Clone)]
pub struct LocalFileHeader {
    version: [u8; 2],
    flags: u16,
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
    pub fn new(file_metadata: &Metadata, target_file_name: &str, compression_method_flag: u8) -> Self {
        let file_name: Vec<u8> = target_file_name.as_bytes().to_vec();
        let file_name_length = file_name.len() as u16;
        let extra_field: Vec<u8> = Vec::new();
        let last_modification_timestamp: u64 = match file_metadata.modified() {
            Ok(time) => time.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            Err(e) => {
                eprintln!("Error: Failed to get file {} last modification datetime: {}", target_file_name, e);
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() // 使用当前时间作为文件最后修改时间的时间戳
            },
        };

        let datetime = DateTime::from(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(last_modification_timestamp));
        Self {
            version: [0x45, 0x00], // 使用zip64格式
            flags: 0b0000000000000100, // 无加密、有数据描述符
            compression_method: [compression_method_flag, 0x00],
            last_modification_date: datetime.msdos_date().to_le_bytes(),
            last_modification_time: datetime.msdos_time().to_le_bytes(),
            crc32: [0x00; 4],
            compressed_size: [0xff, 0xff, 0xff, 0xff], // zip64格式中被设定为0xffffffff
            uncompressed_size: [0xff, 0xff, 0xff, 0xff], // zip64格式中被设定为0xffffffff
            file_name_length: file_name_length.to_le_bytes(),
            extra_field_length: [0x00, 0x00],
            file_name: target_file_name.into(),
            extra_field,
        }
        
    }
}



impl Into<Vec<u8>> for LocalFileHeader {
    fn into(self) -> Vec<u8> {
        let mut local_file_header = Vec::new();
        local_file_header.extend(&0x04034b50u32.to_le_bytes());
        local_file_header.extend(&self.version);
        local_file_header.extend(&self.flags.to_le_bytes());
        local_file_header.extend(&self.compression_method);
        local_file_header.extend(&self.last_modification_time);
        local_file_header.extend(&self.last_modification_date);
        local_file_header.extend(&self.crc32);
        local_file_header.extend(&self.compressed_size);
        local_file_header.extend(&self.uncompressed_size);
        local_file_header.extend(&self.file_name_length);
        local_file_header.extend(&self.extra_field_length);
        local_file_header.extend(&self.file_name);
        local_file_header.extend(&self.extra_field);
        local_file_header
    }
}