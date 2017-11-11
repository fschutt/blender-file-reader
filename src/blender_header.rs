use errors::Error as BlenderError;
use byteorder::{ReadBytesExt, WriteBytesExt};

/// First bytes of a blender file - file header
#[derive(Debug, Clone)]
pub struct BlenderHeader {
    /// File identifier, characters, must be "BLENDER"
    pub file_identifier: [u8; 7],
    /// Pointer size (32 or 64 bit)
    ///
    /// '_' means 4 bytes or 32 bit and '-' means 8 bytes or 64 bits
    pub pointer_size: char,
    /// Type of byte ordering used;
    ///
    /// 'v' means little endian and 'V' means big endian
    pub endianness: char,
    /// Version of Blender the file was created in;
    /// '248' means version 2.48
    pub blender_version: [u8; 3],
}

impl BlenderHeader {
    /// Reads a header. The magic identifer is not checked
    pub fn read<R: ::std::io::Read>(buffer: &mut R) -> Result<Self, BlenderError> {
        let mut file_identifier = [0_u8; 7];
        buffer.read_exact(&mut file_identifier)?;
        
        Ok(BlenderHeader {
            file_identifier: file_identifier,
            pointer_size: buffer.read_u8()? as char,
            endianness: buffer.read_u8()? as char,
            blender_version: [
                buffer.read_u8()?,
                buffer.read_u8()?,
                buffer.read_u8()?,
            ],
        })
    }
    
    pub fn write<W: ::std::io::Write>(&self, target: &mut W) -> Result<(), BlenderError> {
        target.write_u8(self.file_identifier[0])?;
        target.write_u8(self.file_identifier[1])?;
        target.write_u8(self.file_identifier[2])?;
        target.write_u8(self.file_identifier[3])?;
        target.write_u8(self.file_identifier[4])?;
        target.write_u8(self.file_identifier[5])?;
        target.write_u8(self.file_identifier[6])?;
        target.write_u8(self.pointer_size as u8)?;
        target.write_u8(self.endianness as u8)?;
        target.write_u8(self.blender_version[0])?;
        target.write_u8(self.blender_version[1])?;
        target.write_u8(self.blender_version[2])?;
        Ok(())
    }   
}
