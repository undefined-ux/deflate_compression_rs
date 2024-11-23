#[derive(Clone)]
pub struct FileDescriptor {
    crc32: u32,
    compressed_size: u64,
    uncompressed_size: u64,
}


impl FileDescriptor {
    pub fn new(crc32: u32, compressed_size: u64, uncompressed_size: u64) -> Self {
        Self {
            crc32,
            compressed_size,
            uncompressed_size,
        }
    }
}


impl Into<Vec<u8>> for FileDescriptor {
    fn into(self) -> Vec<u8> {
        let mut descriptor = Vec::new();
        descriptor.extend(self.crc32.to_le_bytes().iter());
        descriptor.extend(self.compressed_size.to_le_bytes().iter());
        descriptor.extend(self.uncompressed_size.to_le_bytes().iter());
        descriptor
    }
}