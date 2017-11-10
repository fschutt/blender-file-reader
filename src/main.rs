#[macro_use]
extern crate blender_derive;

#[repr(C, packed)]
#[derive(Debug, BlenderRead, BlenderWrite)]
struct Configuration {
   header: [char; 7],
   pointer_size: char,
   endianness: char,
   version: [char; 3],
}

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
