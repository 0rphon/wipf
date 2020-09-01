#![feature(proc_macro_hygiene, decl_macro, option_unwrap_none)]
#[macro_use] extern crate rocket;

mod wikipf;

use std::path::{PathBuf, Path};

use rocket::response::NamedFile;
use rocket_contrib::json::Json;

#[get("/site/<path..>")]
fn index(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("site/").join(path)).ok()
}

#[get("/api/<page>")]
fn api(page: String) -> Json<wikipf::ApiResp>{
    println!("User searched: {}",page);
    Json(wikipf::get_analytics(page))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, api])
        .launch();
}

//site has stuff to ask api for json data
//api takes target page and flags then scans it and returns analytics in json format
//site then updates a chart to display analytics
//use hashset then serialize to json?? unless fields need to be known

//random    https://en.wikipedia.org/wiki/Special:Random
//test      https://en.wikipedia.org/wiki/Communism

