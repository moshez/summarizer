#![feature(proc_macro_hygiene, decl_macro)]


use rocket::http::RawStr;
//use serde::Serialize;
use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel;

use summarizer::models;
use summarizer::schema;


#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[database("people")]
struct PeopleDBConn(diesel::mysql::MysqlConnection);


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(conn: PeopleDBConn, name: &RawStr) -> Json<String> {
	let useful_name: String = name.as_str().into();
	Json(useful_name)
}

fn main() {
    rocket::ignite()
             .attach(PeopleDBConn::fairing())
             .mount("/", routes![index, hello]).launch();
}
