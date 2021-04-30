// functions for working with TagEntrys

use std::collections::{HashMap,HashSet};
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::datafile::entry_data_file::getDataFile;
use crate::datafile::tag_file::getTagDataFile;
use crate::datafile::tag_schema_file::getTagSchemaFile;

use crate::types::file_types::{EntryDataFile,TagDataFile,TagSchemaFile,TagDataMap};
use crate::types::tag_types::{TagEntries,TagDescriptor};
use crate::types::entry_data_types::EntryData;
use crate::types::tag_types::{TagEntry,TagData};

/// retrieve tag entries using associated files
pub fn getTagEntrys(entrydataPath:&str,tagdataPath:&str,tagschemaPath:&str)->TagEntries
{
    // retrieve all data files
    let entries:EntryDataFile=getDataFile(&entrydataPath);
    let tagdata:TagDataFile=getTagDataFile(&tagdataPath);
    let schemadata:TagSchemaFile=getTagSchemaFile(&tagschemaPath);

    // convert some data files to derived container versions
    let schemadataSet:HashSet<String>=tagSchemaToSet(schemadata);
    let tagdatamap:TagDataMap=mapTagDataFile(tagdata);

    // keep track of seen entry datas
    let mut seenLinkKeys:HashSet<String>=HashSet::new();

    return entries.into_iter().filter_map(|x:EntryData|->Option<TagEntry> {
        // only take one of each link from data
        if seenLinkKeys.contains(&x.link)
        {
            return None;
        }

        seenLinkKeys.insert(x.link.clone());

        let tagData:TagData=match tagdatamap.get(&x.link) {
            Some(r)=>r.clone(),
            None=>TagData::default()
        };

        let missingTags:Vec<String>=determineMissingTags(&tagData,&schemadataSet);

        return Some(TagEntry {
            outdated:missingTags.len()>0,
            numberOutdated:missingTags.len() as u32,
            missingTags,
            tagData,
            data:x
        });
    }).collect();
}

/// return a random tag entry that has missing tags
pub fn randomTagEntry(entrydataPath:&str,tagdataPath:&str,tagschemaPath:&str)->Option<TagEntry>
{
    let entries:TagEntries=getTagEntrys(entrydataPath,tagdataPath,tagschemaPath);

    return match entries.into_iter().filter(|x:&TagEntry|->bool {
        return x.outdated;
    }).collect::<TagEntries>().choose(&mut thread_rng())
    {
        Some(r)=>Some(r.clone()),
        None=>None
    };
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

    return tagset2.difference(&tagdataTags).map(|x:&&String|->String {
        return (*x).clone();
    }).collect();
}

/// convert the tag schema vector to Set. consumes tag schema.
fn tagSchemaToSet(tagschema:TagSchemaFile)->HashSet<String>
{
    return tagschema.into_iter().fold(
        HashSet::new(),
        |mut r:HashSet<String>,x:TagDescriptor|->HashSet<String> {
            r.insert(x.name);
            return r;
        }
    );
}

pub mod tests
{
    use super::getTagEntrys;

    pub fn test()
    {
        let a=getTagEntrys(
            "data/testdata.json",
            "data/test-tagdata.json",
            "data/test-tagschema.yaml"
        );

        println!("{:#?}",a);
    }
}