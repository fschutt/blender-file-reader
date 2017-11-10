#[macro_use]
extern crate blender_derive;

#[repr(C, packed)]
#[derive(Debug, BlenderRead, BlenderWrite)]
struct Configuration {
   item1: u8,
   item2: u16,
   item3: i32,
   item4: [char; 8]
}

trait BlenderRead {
    fn read_big_endian<R: ::std::io::Read>(data: R) -> Self;
    fn read_little_endian<R: ::std::io::Read>(data: R) -> Self;
}

trait BlenderWrite {
    fn write_big_endian<W: ::std::io::Write>(&self, target: W) -> Result<(), ::std::io::Error>;
    fn write_little_endian<W: ::std::io::Write>(&self, target: W) -> Result<(), ::std::io::Error>;
}

const CONFIG_DATA: &'static [u8] = &[
  0xfd, 0xb4, 0x50, 0x45, 0xcd, 0x3c, 0x15, 0x71, 0x3c, 0x87, 0xff, 0xe8,
  0x5d, 0x20, 0xe7, 0x5f, 0x38, 0x05, 0x4a, 0xc4, 0x58, 0x8f, 0xdc, 0x67,
  0x1d, 0xb4, 0x64, 0xf2, 0xc5, 0x2c, 0x15, 0xd8, 0x9a, 0xae, 0x23, 0x7d,
  0xce, 0x4b, 0xeb
];

fn main() {
    let mut config = CONFIG_DATA;
    let data = Configuration::read_big_endian(&mut config);
    println!("Read structure: {:#?}", data);
}
