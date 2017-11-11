#[macro_use]
extern crate blender_derive;
#[macro_use]
extern crate error_chain;
extern crate byteorder;

pub mod errors;
pub mod traits;
pub mod blender_file;
pub mod blender_file_block;
pub mod blender_header;
pub mod blender_sdna;

use blender_file::BlenderFile;

fn main() {    
    let mut test_file = ::std::fs::File::open("assets/default_cube.blend").unwrap();
    let blender_file = BlenderFile::new(&mut test_file).unwrap();
    println!("File: {:#?}", blender_file);
    let mut new_file = ::std::fs::File::create("test.blend").unwrap();
    blender_file.save(&mut new_file).unwrap();
}
