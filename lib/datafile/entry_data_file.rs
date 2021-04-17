// functions for manipulating entry data json file

use std::fs::File;
use std::io::BufReader;

use crate::types::file_types::EntryDataFile;

/// get the entry data file at a certain path.
pub fn getDataFile(path:&str)->EntryDataFile
{
    let file:File=File::open(path).unwrap();
    let reader:BufReader<File>=BufReader::new(file);

    let datafile:EntryDataFile=serde_json::from_reader(reader).unwrap();

    return datafile;
}

pub mod tests
{
    use super::getDataFile;

    pub fn testentrydatafile()
    {
        let datafile=getDataFile("data/testdata.json");
        println!("{}",datafile.len());
    }
}