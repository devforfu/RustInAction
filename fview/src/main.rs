use std::fs::File;
use std::io::prelude::*;
use std::env;

const BYTES_PER_LINE: usize = 16;

fn main() {
    let arg = env::args().nth(1);

    let filename = arg.expect("usage: fview FILENAME");

    let mut f = File::open(&filename).expect("Unable to open file.");
    let mut pos = 0;
    let mut buf = [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buf) {
        print!("[0x{:08x}] ", pos);
        buf.iter().for_each(|byte| {
            match byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        });
        println!();
        pos += BYTES_PER_LINE;
    }
}