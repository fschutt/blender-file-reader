use errors::Error as BlenderError;

pub trait BlenderRead {
    fn read_big_endian<R: ::std::io::Read>(data: &mut R) -> Result<Self, BlenderError> where Self: Sized;
    fn read_little_endian<R: ::std::io::Read>(data: &mut R) -> Result<Self, BlenderError> where Self: Sized;
}

pub trait BlenderWrite {
    fn write_big_endian<W: ::std::io::Write>(&self, target: &mut W) -> Result<(), BlenderError>;
    fn write_little_endian<W: ::std::io::Write>(&self, target: &mut W) -> Result<(), BlenderError>;
}
