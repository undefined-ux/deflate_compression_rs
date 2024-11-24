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
    assert_eq!(crc32, 0x7A2A208B, "CRC32 is not correct {:#X} != {:#X}", crc32, 0x7A2A208B);
}

#[allow(dead_code)]
fn generate_crc32_table() {
    let poly: u32 = 0xedb88320;
    let mut table: [u32; 256] = [0; 256];
    for i in 0..256 {
        let mut crc = i as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ poly
            } else {
                crc >> 1
            }
        }
        table[i] = crc.to_le() as u32;
    };
    println!("{:#X?}", table);
}