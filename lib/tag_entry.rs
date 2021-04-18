// functions for working with TagEntrys

use std::collections::HashMap;

use crate::datafile::entry_data_file::getDataFile;
use crate::datafile::tag_file::getTagDataFile;
use crate::datafile::tag_schema_file::getTagSchemaFile;

use crate::types::file_types::{EntryDataFile,TagDataFile,TagSchemaFile,TagDataMap};
use crate::types::tag_types::TagEntries;
use crate::types::entry_data_types::EntryData;
use crate::types::tag_types::{TagEntry,TagData};

fn getTagEntrys(entrydataPath:&str,tagdataPath:&str,tagschemaPath:&str)->TagEntries
{
    let entries:EntryDataFile=getDataFile(&entrydataPath);
    let tagdata:TagDataFile=getTagDataFile(&tagdataPath);
    let schemadata:TagSchemaFile=getTagSchemaFile(&tagschemaPath);

    let tagdatamap:TagDataMap=mapTagDataFile(tagdata);

    println!("{:#?}",tagdatamap);

    return vec![];
}

/// convert TagDataFile into TagDataMap. CONSUMES the tag data.
fn mapTagDataFile(tagData:TagDataFile)->TagDataMap
{
    return tagData.into_iter().fold(
        HashMap::new(),
        |mut r:TagDataMap,x:TagData|->TagDataMap {
            r.insert(x.link.clone(),x);
            return r;
        }
    );
}

pub mod tests
{
    use super::getTagEntrys;

    pub fn test()
    {
        getTagEntrys(
            "data/testdata.json",
            "data/test-tagdata.json",
            "data/test-tagschema.yaml"
        );
    }
}