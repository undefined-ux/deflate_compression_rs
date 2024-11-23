use std::path::{Path, PathBuf};

pub mod deflate;
pub mod no_compress;


pub struct CompressResult {
    // 在压缩过程中直接计算crc32, 节省时间
    pub data: Vec<u8>,
    pub crc32: u32,
    pub compressed_size: u64,
}


pub trait Compressable {
    fn compress_data(&self, file: &Path) -> Result<CompressResult, ()> ;
    fn compress_method(&self) -> u8;
    fn compress_method_name(&self) -> String;
}