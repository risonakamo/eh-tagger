// schema types for the data files

use super::tag_types::{TagData,TagDescriptor};
use super::entry_data_types::EntryData;

/// tag data file is array of TagData objects
pub type TagDataFile=Vec<TagData>;

/// entry data file is array of EntryData objects
pub type EntryDataFile=Vec<EntryData>;

/// tag schema file is array of TagDescriptor objects
pub type TagSchemaFile=Vec<TagDescriptor>;