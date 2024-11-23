#[test]
fn test_can_calcualte_crc32() {
    let data = b"hello world";
    let crc32 = crate::utils::crc32::Crc32::calculate(data);
    assert_eq!(crc32, 0x8a9136aa);
}