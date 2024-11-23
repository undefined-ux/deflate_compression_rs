use crate::utils::datetime::DateTime;
#[test]
fn test_datetime_local() {
    let datetime = DateTime::local();
    println!("Current local datetime: {}", datetime);
}

#[test]
fn test_utc_datetime() {
    let datetime = DateTime::utc();
    println!("Current UTC datetime: {}", datetime);
}

#[test]
fn test_datetime_msdos_format_date() {
    let datetime = DateTime::local();
    let msdos_date = datetime.msdos_date();
    println!(
        "MS-DOS format date: {:#b} {:04}/{:02}/{:02}",
        msdos_date,
        (msdos_date >> 9) as u32 + 1980,
        (msdos_date >> 5) & 0xF,
        msdos_date & 0x1F
    );
    assert_eq!(msdos_date & 0x1F, datetime.day as u16);
    assert_eq!((msdos_date >> 5) & 0xF, datetime.month as u16);
    assert_eq!((msdos_date >> 9) as u32 + 1980, datetime.year as u32);
}

#[test]
fn test_datetime_msdos_format_time() {
    let datetime = DateTime::local();
    let msdos_time = datetime.msdos_time();
    println!(
        "Current DateTime: {}\nMS-DOS format time: {:#b} {:02}:{:02}:{:02}",
        datetime,
        msdos_time,
        (msdos_time >> 11) & 0x1f,
        (msdos_time >> 5) & 0x3f,
        msdos_time & 0xF
    );
    assert_eq!(msdos_time & 0x1F, (datetime.second / 2) as u16);
    assert_eq!((msdos_time >> 5) & 0x3f, datetime.minute as u16);
    assert_eq!((msdos_time >> 11) & 0x1f, datetime.hour as u16);
}
