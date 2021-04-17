/// !! CROSS LANGUAGE TYPES
/// !! mirrors tag-types.d.ts

use std::collections::HashMap;
use serde::{Deserialize,Serialize};

use super::entry_data_types::EntryData;

/** key: tag name, val: tag present or not */
pub type TagSet=HashMap<String,bool>;

/** data object representing a set of tags associated with a data entry */
#[derive(Deserialize,Serialize,Debug)]
pub struct TagData
{
    pub link:String,
    pub tags:TagSet
}

/** represents a tag in a tag schema */
#[derive(Deserialize,Serialize,Debug)]
pub struct TagDescriptor
{
    pub name:String,
    #[serde(default)]
    pub description:String
}

/** derived data object representing combined entry data and associated tag data. */
pub struct TagEntry
{
    pub data:EntryData,
    pub tagData:TagData,

    pub missingTags:Vec<String>,

    pub outdated:bool,
    pub numberOutdated:u32
}

/** vector of TagEntries. could be replaced by a hashmap sometime. */
pub type TagEntries=Vec<TagEntry>;