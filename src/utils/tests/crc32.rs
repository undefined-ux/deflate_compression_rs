use std::path::Path;

#[test]
fn test_can_calcualte_crc32() {
    let path = Path::new("Cargo.toml");
    let crc32 = match crate::utils::crc32::Crc32::calculate_file(path) {
        Ok(val) => val,
        Err(_) => {
            assert!(true, "Failed to calculate crc32");
            panic!()
        },
    };
    assert_eq!(crc32, 0x7A2A208B, "CRC32 is not correct {} != {}", crc32, 0x7A2A208B);
}