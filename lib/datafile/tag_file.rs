// functions for manipulating tag data file json

use std::fs::File;
use std::io::{BufReader,BufWriter};

use crate::types::file_types::TagDataFile;
use crate::types::tag_types::{TagUpdate,TagData};

/// get the get tag data file from path.
pub fn getTagDataFile(path:&str)->TagDataFile
{
    let file:File=File::open(path).unwrap();
    let reader:BufReader<File>=BufReader::new(file);

    let datafile:TagDataFile=serde_json::from_reader(reader).unwrap();

    return datafile;
}

/// update tag file with tag update. writes to the file. consumes tag update
pub fn updateTagFile(path:&str,update:TagUpdate)
{
    let mut tagfile:TagDataFile=getTagDataFile(path);

    let item:Option<&mut TagData>=tagfile.iter_mut().find(|x:&&mut TagData|->bool {
        return x.link==update.link;
    });

    match item
    {
        Some(r)=>{
            r.tags.extend(update.tags);
        }

        None=>{
            tagfile.push(TagData {
                link:update.link,
                tags:update.tags
            });
        }
    }

    writeTagDataFile(path,tagfile);
}

/// write out the tag data file to the path. consumes.
fn writeTagDataFile(path:&str,tagfile:TagDataFile)
{
    let file:File=File::create(path).unwrap();
    let writer:BufWriter<File>=BufWriter::new(file);

    serde_json::to_writer_pretty(writer,&tagfile).unwrap();
}

pub mod tests
{
    use super::{getTagDataFile,updateTagFile};
    use super::TagUpdate;

    pub fn test()
    {
        let a=getTagDataFile("data/test-tagdata.json");
        println!("{:#?}",&a);
    }

    pub fn test2()
    {
        updateTagFile("data/test-tagdata.json",TagUpdate {
            link:"a new key2".to_string(),
            tags:vec![
                ("something".to_string(),false),
                ("something2".to_string(),false),
                ("something".to_string(),true),
                ("something3".to_string(),true)
            ].into_iter().collect()
        });
    }
}