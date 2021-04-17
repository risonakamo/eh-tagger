// functions for manipulating tag schema yaml file

use std::fs::File;
use std::io::BufReader;

use crate::types::file_types::TagSchemaFile;

pub fn getTagSchemaFile(path:&str)->TagSchemaFile
{
    let file:File=File::open(path).unwrap();
    let reader:BufReader<File>=BufReader::new(file);

    let datafile:TagSchemaFile=serde_yaml::from_reader(reader).unwrap();

    return datafile;
}

pub mod tests
{
    use super::getTagSchemaFile;

    pub fn test()
    {
        let a=getTagSchemaFile("data/test-tagschema.yaml");
        println!("{:#?}",&a);
    }
}