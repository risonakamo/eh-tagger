/// !! CROSS LANGUAGE TYPES
/// !! mirrors tag-types.d.ts

use std::collections::HashMap;

/** key: tag name, val: tag present or not */
pub type TagSet=HashMap<String,bool>;

/** data object representing a set of tags associated with a data entry */
pub struct TagData
{
    pub link:String,
    pub tags:TagSet
}

/** represents a tag in a tag schema */
pub struct TagDescriptor
{
    pub name:String,
    pub description:String
}

/** data object representing combined entry data and associated tag data. */
pub struct TagEntry
{
    // pub data:
    pub tagData:TagData,

    pub missingTags:Vec<String>,

    pub outdated:bool,
    pub numberOutdated:u32
}