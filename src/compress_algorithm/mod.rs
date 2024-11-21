use std::path::{Path, PathBuf};

pub mod deflate;

pub trait Compressable {
    fn compress_data(&self, file: &Path) -> Result<Vec<u8>, ()> ;
    fn compress_method(&self) -> u8;
    fn compress_method_name(&self) -> String;
}