use std::collections::{HashMap, HashSet};
use errors::Error as BlenderError;
use blender_file_block::BlenderFileBlock;

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
    pub fn from_blocks(blocks: &Vec<BlenderFileBlock>) -> Result<Self, BlenderError> {
        use std::ffi::CString;
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
    pub fn access_field(struct_name: &str) -> Option<BlenderStructInfo> {
        // TODO!!
        None
    }
}
