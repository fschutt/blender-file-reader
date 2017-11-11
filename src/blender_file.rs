use blender_sdna::BlenderSdna;
use blender_header::BlenderHeader;
use blender_file_block::BlenderFileBlock;

use errors::Error as BlenderError;
use errors::ErrorKind as BlenderErrorKind;

pub const BLENDER_MAGIC_HEADER: [u8; 7] =
    ['B' as u8,
     'L' as u8,
     'E' as u8,
     'N' as u8,
     'D' as u8,
     'E' as u8,
     'R' as u8];

/// File holding the data for the individual components
#[derive(Debug, Clone)]
pub struct BlenderFile {
    pub header: BlenderHeader,
    pub file_blocks: Vec<BlenderFileBlock>,
    pub sdna: BlenderSdna,
}

impl BlenderFile {

    /// Read the file from a data source
    pub fn new<R: ::std::io::Read>(source: &mut R) -> Result<Self, BlenderError> {
        let header = BlenderHeader::read(source)?;
        if header.file_identifier != BLENDER_MAGIC_HEADER {
            return Err(BlenderError::from(BlenderErrorKind::InvalidBlendFile));
        }

        let file_blocks = Self::split_into_file_blocks(source, &header)?;
        let sdna_block = BlenderSdna::from_blocks(&file_blocks)?;
        
        Ok(Self {
            header: header,
            file_blocks: file_blocks,
            sdna: sdna_block,
        })
    }

    /// Splits the file into file blocks.
    /// NOTE: The read head should be at the end of the header
    fn split_into_file_blocks<R: ::std::io::Read>(source: &mut R, header: &BlenderHeader) -> Result<Vec<BlenderFileBlock>, BlenderError> {
        // TODO! 
        Ok(Vec::new())
    }
    
    /// Save the file
    pub fn save<W: ::std::io::Write>(&self, target: &mut W) -> Result<(), BlenderError> {
        // TODO!
        Ok(())
    } 
}
