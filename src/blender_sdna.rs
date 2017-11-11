use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

use errors::Error as BlenderError;
use blender_file::MAGIC_SDNA;
use blender_file_block::BlenderFileBlock;
use blender_header::{Endianness, BlenderHeader};

#[derive(Debug, Clone)]
pub struct BlenderSdna {
    /// Names of the variables, may contain pointer
    /// e.g. "
    pub names: HashSet<String>,
    /// Name of the type and length in bytes
    /// e.g. "int", "4"
    pub types: HashMap<String, u16>,
    /// <Index of structure name in self.types,
    ///  Vec of Fields: (Index of type, Index of field)>
    pub structs: HashMap<u16, Vec<(u16, u16)>>,
}

pub struct BlenderStructInfo {
    pub struct_name: String,
    pub fields: Vec<(String, u16)>,
    pub begin_in_file: u64,
}

impl BlenderSdna {
    /// Searches the file blocks for "DNA1" file block, extracts the
    /// relevant types, structs and names
    pub fn from_blocks(_blocks: &Vec<BlenderFileBlock>) -> Result<Self, BlenderError> {

        let names = HashSet::new();
        let types = HashMap::new();
        let structs = HashMap::new();

        Ok(Self {
            names: names,
            types: types,
            structs: structs,
        })
    }

    /// Calculates the offset in the original file (from the beginning) where the 
    pub fn access_field(_struct_name: &str) -> Option<BlenderStructInfo> {
        // TODO!!
        None
    }

    pub fn write<W: ::std::io::Write>(&self, target: &mut W, header: &BlenderHeader) -> Result<(), BlenderError> {
        target.write(&MAGIC_SDNA)?;
        target.write(&['N' as u8, 'A' as u8, 'M' as u8, 'E' as u8])?; // TODO: this is not correct

        // write names
        match header.endianness {
            Endianness::Big => { target.write_u32::<BigEndian>(self.names.len() as u32)?; },
            Endianness::Little => { target.write_u32::<LittleEndian>(self.names.len() as u32)?; },
        }
        
        for name in self.names.iter() {
            target.write(&CString::new(name.clone())?.into_bytes())?;
        }

        // TODO: pad to 4 bytes
        target.write(&['T' as u8, 'Y' as u8, 'P' as u8, 'E' as u8])?;

        // write types
        match header.endianness {
            Endianness::Big => { target.write_u32::<BigEndian>(self.types.len() as u32)?; },
            Endianness::Little => { target.write_u32::<LittleEndian>(self.types.len() as u32)?; },
        }
        
        for key in self.types.keys() {
            target.write(&CString::new(key.clone())?.into_bytes())?;
        }
        
        target.write(&['T' as u8, 'L' as u8, 'E' as u8, 'N' as u8])?;

        for val in self.types.values() {
            match header.endianness {
                Endianness::Big => { target.write_u16::<BigEndian>(*val)?; },
                Endianness::Little => { target.write_u16::<LittleEndian>(*val)?; },
            }
        }
        target.write(&['S' as u8, 'T' as u8, 'R' as u8, 'C' as u8])?;
        
        // write structures
        match header.endianness {
            Endianness::Big => { target.write_u32::<BigEndian>(self.structs.len() as u32)?; },
            Endianness::Little => { target.write_u32::<LittleEndian>(self.structs.len() as u32)?; },
        }

        for (key, val) in self.structs.iter() {

            match header.endianness {
                Endianness::Big => {
                    target.write_u16::<BigEndian>(*key)?;
                    target.write_u16::<BigEndian>(val.len() as u16)?;
                },
                Endianness::Little => {
                    target.write_u16::<LittleEndian>(*key)?;
                    target.write_u16::<LittleEndian>(val.len() as u16)?;
                },
            }
            for &(ref field_type_id, ref field_name_id) in val.iter() {
                match header.endianness {
                    Endianness::Big => { target.write_u16::<BigEndian>(*field_type_id)?; },
                    Endianness::Little => { target.write_u16::<LittleEndian>(*field_name_id)?; },
                }
            }
        }
        
        Ok(())
    }
}
