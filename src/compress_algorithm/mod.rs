use std::path::{Path, PathBuf};

pub mod deflate;
pub mod no_compress;


pub struct CompressResult {
    pub data: Vec<u8>,
    pub compressed_size: u64,
}


pub trait Compressable {
    fn compress_data(&self, file: &Path) -> Result<CompressResult, ()> ;
    fn compress_method(&self) -> u8;
    fn compress_method_name(&self) -> String;
}