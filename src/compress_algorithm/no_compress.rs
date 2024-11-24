use std::{fs::File, io::Read, path::Path};
use crate::compress_algorithm::Compressable;

use super::CompressResult;

pub struct NoCompress;


impl Compressable for NoCompress {
    fn compress_data(&self, file_path: &Path) -> Result<CompressResult, ()> {
        let file = File::open(file_path).unwrap();
        let mut file_data = Vec::new();
        file.take(10).read_to_end(&mut file_data).unwrap();
        Ok(CompressResult {
            data: file_data.clone(),
            compressed_size: file_data.len() as u64,
        })
    }
    
    fn compress_method(&self) -> u8 {
        0
    }
    
    fn compress_method_name(&self) -> String {
        "No Compression".to_string()
    }
}