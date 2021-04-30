#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::{get,post};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{JsonValue,Json};
use rocket_contrib::json;

use eh_tagger::tag_entry::{getTagEntrys,randomTagEntry};
use eh_tagger::datafile::tag_file::updateTagFile;

use eh_tagger::types::tag_types::TagUpdate;

/// get tag entries api. retrieves tag entries.
#[get("/get-tag-entries")]
fn api_getTagEntries()->JsonValue
{
    // TODO: make these configurable paths
    return json!(getTagEntrys(
        "data/testdata.json",
        "data/test-tagdata.json",
        "data/test-tagschema.yaml"
    ));
}

/// get random entry with missing tags.
#[get("/get-random-entry")]
fn api_getRandomEntry()->JsonValue
{
    // TODO: make these configurable paths
    return json!(randomTagEntry(
        "data/testdata.json",
        "data/test-tagdata.json",
        "data/test-tagschema.yaml"
    ));
}

/// update the tag file.
#[post("/update-tag",format="json",data="<request>")]
fn api_updateTag(request:Json<TagUpdate>)->JsonValue
{
    let tagupdate:TagUpdate=request.into_inner();
    // TODO: make this configurable
    updateTagFile("data/test-tagdata.json",tagupdate);
    return json!("complete");
}

fn main()
{
    rocket::ignite()
        .mount("/",StaticFiles::from("web"))
        .mount("/",rocket::routes![api_getTagEntries,api_updateTag,api_getRandomEntry])
        .launch();
}