use errors::Error as BlenderError;
use blender_header::{PointerSize, Endianness, BlenderHeader};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

#[derive(Debug, Clone)]
pub struct BlenderFileBlock {
    /// Identifier of the file-block
    /// The last file block has the ID 'ENDB'.
    /// SC00 for scene
    pub code: [u8; 4],
    /// Total length of the data after the file-block-header
    pub size: u32,
    /// Old memory address as void*
    /// TODO: This pointer is pointer-size, needs the header to be computed
    /// This is u64 just for quick testing!!! 
    pub old_pointer: u64,
    /// SDNA index - Index of the SDNA structure
    pub sdna_index: u32,
    /// The type of this structure.
    /// There is an id for lights, materials, scenes, etc.
    pub type_id: u32,
    /// The data for this file block. This can be anything
    pub data: Vec<u8>,
}

impl BlenderFileBlock {
    
    /// Loads the data for a file block
    pub fn load_from_source<R: ::std::io::Read>(_source: &mut R, _header: &BlenderHeader) -> Result<Self, BlenderError> {
        // TODO!!
        Ok(Self {
            code: [0; 4],
            size: 0,
            old_pointer: 0,
            sdna_index: 0,
            type_id: 0,
            data: Vec::new(),
        })
    }

    /// Saves the data in the file block
    pub fn write<W: ::std::io::Write>(&self, target: &mut W, header: &BlenderHeader) -> Result<(), BlenderError> {
        target.write(&self.code)?;
        match header.endianness {
            Endianness::Big =>   {
                target.write_u32::<BigEndian>(self.size)?;
                match header.pointer_size {
                    PointerSize::Big =>   { target.write_u64::<BigEndian>(self.old_pointer)?; },
                    PointerSize::Small => { target.write_u32::<BigEndian>(self.old_pointer as u32)?; },
                }
                target.write_u32::<BigEndian>(self.sdna_index)?;
                target.write_u32::<BigEndian>(self.type_id)?;
            },
            Endianness::Little => {
                target.write_u32::<LittleEndian>(self.size)?;
                match header.pointer_size {
                    PointerSize::Big =>   { target.write_u64::<LittleEndian>(self.old_pointer)?; },
                    PointerSize::Small => { target.write_u32::<LittleEndian>(self.old_pointer as u32)?; },
                }
                target.write_u32::<LittleEndian>(self.sdna_index)?;
                target.write_u32::<LittleEndian>(self.type_id)?;
            },
        }

        target.write(&self.data)?;
        Ok(())
    }
}
