use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

fn main() -> Result<(), std::io::Error> {
    let ((one, two, three), buf) = write_numbers_to_file()?;
    let (one_, two_, three_) = read_numbers_from_file(&buf)?;
    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);
    Ok(())
}

fn write_numbers_to_file() -> Result<((u32, i8, f64), Vec<u8>), std::io::Error> {
    let mut w = vec![];

    let one: u32 = 1;
    let two: i8 = 2;
    let three: f64 = 3.0;

    w.write_u32::<LittleEndian>(one)?;
    println!("{:?}", &w);

    w.write_i8(two)?;
    println!("{:?}", &w);

    w.write_f64::<LittleEndian>(three)?;
    println!("{:?}", &w);

    println!("Parity bit: {}", parity_bit(&w));

    Ok(((one, two, three), w))
}

fn read_numbers_from_file(buf: &Vec<u8>) -> Result<(u32, i8, f64), std::io::Error> {
    let mut cur = Cursor::new(buf);
    let one = cur.read_u32::<LittleEndian>()?;
    let two = cur.read_i8()?;
    let three = cur.read_f64::<LittleEndian>()?;
    Ok((one, two, three))
}

fn parity_bit(bytes: &[u8]) -> u8 {
    let mut n_ones: u32 = 0;
    for byte in bytes {
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b}) has {} one bits", byte, byte, ones);
    }
    (n_ones % 2 == 0) as u8
}