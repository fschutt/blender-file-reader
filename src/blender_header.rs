use errors::Error as BlenderError;
use errors::ErrorKind as BlenderErrorKind;
use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Debug, Clone)]
pub enum Endianness {
    Big,
    Little,
}

#[derive(Debug, Clone)]
pub enum PointerSize {
    Big,
    Small,
}

/// First bytes of a blender file - file header
#[derive(Debug, Clone)]
pub struct BlenderHeader {
    /// File identifier, characters, must be "BLENDER"
    pub file_identifier: [u8; 7],
    /// Pointer size (32 or 64 bit)
    ///
    /// '_' means 4 bytes or 32 bit and '-' means 8 bytes or 64 bits
    pub pointer_size: PointerSize,
    /// Type of byte ordering used;
    ///
    /// 'v' means little endian and 'V' means big endian
    pub endianness: Endianness,
    /// Version of Blender the file was created in;
    /// '248' means version 2.48
    pub blender_version: [u8; 3],
}

impl BlenderHeader {
    /// Reads a header. The magic identifer is not checked
    pub fn read<R: ::std::io::Read>(buffer: &mut R) -> Result<Self, BlenderError> {
        let mut file_identifier = [0_u8; 7];
        buffer.read_exact(&mut file_identifier)?;

        let ptr_size = buffer.read_u8()? as char;
        let pointer_size = match ptr_size {
            '-' => PointerSize::Big,
            '_' => PointerSize::Small,
            _ => return Err(BlenderError::from(BlenderErrorKind::InvalidPointerSize(ptr_size))),
        };
        
        let endianness = buffer.read_u8()? as char;
        let endian = match endianness {
            'V' => Endianness::Big,
            'v' => Endianness::Little,
                        _ => return Err(BlenderError::from(BlenderErrorKind::InvalidEndianness(endianness))),
        };

        Ok(BlenderHeader {
            file_identifier: file_identifier,
            pointer_size: pointer_size,
            endianness: endian,
            blender_version: [
                buffer.read_u8()?,
                buffer.read_u8()?,
                buffer.read_u8()?,
            ],
        })
    }

    /// Writes the header
    pub fn write<W: ::std::io::Write>(&self, target: &mut W) -> Result<(), BlenderError> {
        target.write(&self.file_identifier)?;

        let pointer_size = match self.pointer_size {
            PointerSize::Big => '-',
            PointerSize::Small => '_',
        };
        
        let endian = match self.endianness {
            Endianness::Big => 'V',
            Endianness::Little => 'v',
        };


        target.write_u8(pointer_size as u8)?;
        target.write_u8(endian as u8)?;
        target.write_u8(self.blender_version[0])?;
        target.write_u8(self.blender_version[1])?;
        target.write_u8(self.blender_version[2])?;
        Ok(())
    }   
}
