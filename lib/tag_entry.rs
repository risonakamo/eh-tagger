// functions for working with TagEntrys

use crate::datafile::entry_data_file::getDataFile;
use crate::datafile::tag_file::getTagDataFile;
use crate::datafile::tag_schema_file::getTagSchemaFile;

use crate::types::file_types::{EntryDataFile,TagDataFile,TagSchemaFile};
use crate::types::tag_types::TagEntries;

fn getTagEntrys(entrydataPath:&str,tagdataPath:&str,tagschemaPath:&str)->TagEntries
{
    let entries:EntryDataFile=getDataFile(&entrydataPath);
    let tagdata:TagDataFile=getTagDataFile(&tagdataPath);
    let schemadata:TagSchemaFile=getTagSchemaFile(&tagschemaPath);


}