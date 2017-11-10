#[macro_use]
extern crate blender_derive;
extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt, LittleEndian, WriteBytesExt};

#[repr(C, packed)]
#[derive(Debug, BlenderRead, BlenderWrite)]
struct Configuration {
    version: [i32; 3],
    pointer_size: u8,
    endianness: u8,
}

// impl BlenderRead for Configuration { fn read_big_endian<R: ::std::io::Read>(mut buffer: R) -> Self { Configuration { version : [ buffer.read_i32::<BigEndian>().unwrap() ,  buffer.read_i32::<BigEndian>().unwrap() ,  buffer.read_i32::<BigEndian>().unwrap() ]; pointer_size : buffer.read_u8().unwrap(); endianness : buffer.read_u8().unwrap(); } } fn read_little_endian<R: ::std::io::Read>(mut buffer: R) -> Self { Configuration { version : [ buffer.read_i32::<LittleEndian>().unwrap() ,  buffer.read_i32::<LittleEndian>().unwrap() ,  buffer.read_i32::<LittleEndian>().unwrap() ]; pointer_size : buffer.read_u8().unwrap(); endianness : buffer.read_u8().unwrap(); } } }

trait BlenderRead {
    fn read_big_endian<R: ::std::io::Read>(data: R) -> Self;
    fn read_little_endian<R: ::std::io::Read>(data: R) -> Self;
}

trait BlenderWrite {
    fn write_big_endian<W: ::std::io::Write>(&self, target: W) -> Result<(), ::std::io::Error>;
    fn write_little_endian<W: ::std::io::Write>(&self, target: W) -> Result<(), ::std::io::Error>;
}

fn main() {
    let mut test_file = ::std::fs::File::open("assets/default_cube.blend").unwrap();
    let data = Configuration::read_big_endian(&mut test_file);
    println!("Read structure: {:#?}", data);
}
