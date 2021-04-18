#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::get;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::JsonValue;
use rocket_contrib::json;

use eh_tagger::tag_entry::getTagEntrys;

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

fn main()
{
    rocket::ignite()
        .mount("/",StaticFiles::from("web"))
        .mount("/",rocket::routes!(api_getTagEntries))
        .launch();
}