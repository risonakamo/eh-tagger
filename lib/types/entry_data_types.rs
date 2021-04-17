// !! CROSS LANGUAGE TYPES
// !! mirrors entry-data-types.d.ts

pub enum EntryType
{
    NHENTAI,OTHER,SANKAKU,IMGUR,DLSITE,HITOMI,PIXIV,EXHENTAI,BETASANKAKU
}

/** current interface for log entry data. */
pub struct EntryData
{
    pub date:String,
    pub group:String,
    pub link:String,
    pub name:String,
    pub r#type:EntryType
}