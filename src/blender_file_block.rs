
use traits::{BlenderRead, BlenderWrite};
use errors::Error as BlenderError;
use blender_header::BlenderHeader;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

/// Ending block, marks end of file
const MAGIC_ENDB: [u8; 4] = ['E' as u8, 'N' as u8, 'D' as u8, 'B' as u8];
/// SDNA block ID, describes the file structure
const MAGIC_SDNA: [u8; 4] = ['D' as u8, 'N' as u8, 'A' as u8, '1' as u8]; 

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
    /// The data for this file block. This can be anything.
    pub data: Vec<u8>,
}

impl BlenderFileBlock {
    
    /// Loads the data for a file block
    fn load_from_source<R: ::std::io::Read>(source: &mut R, header: &BlenderHeader) -> Result<Self, BlenderError> {
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
}
