#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::{get,post};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{JsonValue,Json};
use rocket_contrib::json;

use eh_tagger::tag_entry::getTagEntrys;
use eh_tagger::datafile::tag_file::updateTagFile;

use eh_tagger::types::tag_types::TagUpdate;

/// get tag entries api. retrieves tag entries.
#[get("/get-tag-entries")]
fn api_getTagEntries()->JsonValue
{
    // TODO: make these configurable paths
    json!(getTagEntrys(
        "data/testdata.json",
        "data/test-tagdata.json",
        "data/test-tagschema.yaml"
    ))
}

/// update the tag file.
#[post("/update-tag",format="json",data="<request>")]
fn api_updateTag(request:Json<TagUpdate>)->JsonValue
{
    let tagupdate:TagUpdate=request.into_inner();
    updateTagFile("data/testdata.json",tagupdate);
    return json!("complete");
}

fn main()
{
    rocket::ignite()
        .mount("/",StaticFiles::from("web"))
        .mount("/",rocket::routes![api_getTagEntries,api_updateTag])
        .launch();
}