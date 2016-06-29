extern crate byteorder;

use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
struct Record {
    x: i32,
    y: u16,
    z: i16,
}

fn main() {
    let mut f = File::open("foo.txt").unwrap(); // fail if the file doesn't exist
    let mut buffer = vec![0; 64];

    f.read_exact(&mut buffer).unwrap(); // fail if the read fails

    let mut cursor = Cursor::new(buffer);

    // these will all fail if the parsing fails
    let x = cursor.read_i32::<BigEndian>().unwrap();
    let y = cursor.read_u16::<BigEndian>().unwrap();
    let z = cursor.read_i16::<BigEndian>().unwrap();

    let record = Record { x: x, y: y, z: z };

    println!("Record found: {:?}", record);
}
