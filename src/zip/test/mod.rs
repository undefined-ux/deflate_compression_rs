use crate::{compress_algorithm::no_compress::NoCompress, zip::files::File};

#[test]
fn test_local_file_header() {
    let file = File::new("Cargo.toml", "Cargo.toml", NoCompress).unwrap();
    let local_file_header = file.get_local_file_header();
    println!("{:02X?}", local_file_header);
}