// functions for manipulating tag data file json

use std::fs::File;
use std::io::BufReader;

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

    let item:&mut TagData=tagfile.iter_mut().find(|x:&&mut TagData|->bool {
        return x.link==update.link;
    }).unwrap();

    item.tags.insert("asda".to_string(),false);
    println!("{:#?}",item);
    println!("{:#?}",tagfile);
}

/// write out the tag data file to the path. consumes.
fn writeTagDataFile(path:&str,tagfile:TagDataFile)
{

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
            link:"https://chan.sankakucomplex.com/?tags=neocoill".to_string(),
            tags:vec![
                ("something".to_string(),false)
            ].into_iter().collect()
        });
    }
}