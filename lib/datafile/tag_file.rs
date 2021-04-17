// functions for manipulating tag data file json

use std::fs::File;
use std::io::BufReader;

use crate::types::file_types::TagDataFile;

/// get the get tag data file from path.
pub fn getTagDataFile(path:&str)->TagDataFile
{
    let file:File=File::open(path).unwrap();
    let reader:BufReader<File>=BufReader::new(file);

    let datafile:TagDataFile=serde_json::from_reader(reader).unwrap();

    return datafile;
}

pub mod tests
{
    use super::getTagDataFile;

    pub fn test()
    {
        let a=getTagDataFile("data/test-tagdata.json");
        println!("{:#?}",&a);
    }
}