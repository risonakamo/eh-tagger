// !! CROSS LANGUAGE TYPES
// !! mirrors entry-data-types.d.ts

use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub enum EntryType
{
    NHENTAI,OTHER,SANKAKU,IMGUR,DLSITE,HITOMI,PIXIV,EXHENTAI,BETASANKAKU
}

/** current interface for log entry data. */
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct EntryData
{
    pub date:String,
    pub group:String,
    pub link:String,
    pub name:String,
    pub r#type:EntryType
}