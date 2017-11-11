use errors::Error as BlenderError;
use blender_header::{Endianness, BlenderHeader};

pub trait BlenderRead {
    fn read_big_endian<R: ::std::io::Read>(data: &mut R) -> Result<Self, BlenderError> where Self: Sized;
    fn read_little_endian<R: ::std::io::Read>(data: &mut R) -> Result<Self, BlenderError> where Self: Sized;
    fn read<R: ::std::io::Read>(data: &mut R, header: &BlenderHeader) -> Result<Self, BlenderError> where Self: Sized {
        match header.endianness {
            Endianness::Big => Self::read_big_endian(data),
            Endianness::Little => Self::read_little_endian(data),
        }
    }
}

pub trait BlenderWrite {
    fn write_big_endian<W: ::std::io::Write>(&self, target: &mut W) -> Result<(), BlenderError>;
    fn write_little_endian<W: ::std::io::Write>(&self, target: &mut W) -> Result<(), BlenderError>;
    fn write<W: ::std::io::Write>(&self, target: &mut W, header: &BlenderHeader) -> Result<(), BlenderError> where Self: Sized {
        match header.endianness {
            Endianness::Big => self.write_big_endian(target),
            Endianness::Little => self.write_little_endian(target),
        }
    }
}
