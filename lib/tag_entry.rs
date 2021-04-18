// functions for working with TagEntrys

use std::collections::{HashMap,HashSet};

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

    let mut seenLinkKeys:HashSet<String>=HashSet::new();
    return entries.into_iter().filter_map(|x:EntryData|->Option<TagEntry> {
        // only take one of each link from data
        if seenLinkKeys.contains(&x.link)
        {
            return None;
        }

        seenLinkKeys.insert(x.link.clone());

        return Some(TagEntry {
            tagData:match tagdatamap.get(&x.link) {
                Some(res)=>res.clone(),
                None=>TagData::default()
            },
            data:x,
            missingTags:vec![],
            outdated:false,
            numberOutdated:0
        });
    }).collect();
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

/// given a tag data object, determine what tags it does not have given the tagset as
/// an array of string tags.
fn determineMissingTags(tagdata:&TagData,tagset:&HashSet<String>)->Vec<String>
{
    let tagdataTags:HashSet<&String>=tagdata.tags.keys().collect();
    let tagset2:HashSet<&String>=tagset.into_iter().collect();

    return tagset2.difference(&tagdataTags).into_iter().map(|x:&&String|->String {
        return (*x).clone();
    }).collect();
}

/// convert the tag schema vector to Set
fn tagSchemaToSet(tagschema:TagSchemaFile)->HashSet<String>
{

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