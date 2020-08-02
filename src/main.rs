#![feature(proc_macro_hygiene, decl_macro)]


use rocket::http::RawStr;
//use serde::Serialize;
use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel;

use self::models;
use self::schema;


#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[database("people")]
struct PeopleDBConn(diesel::mysql::MysqlConnection);


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn get_pronouns_from_db(conn: &diesel::mysql::MysqlConnection, name: &String) -> i32 {
	let results = schema::people.filter(schema::people::name.eq(name))
		.limit(1)
		.load::<models::Person>(conn)
		.expect("Woops");
	println!("pretending a little {:?}", results);
	5
}

#[get("/hello/<name>")]
fn hello(conn: PeopleDBConn, name: &RawStr) -> Json<String> {
	let useful_name: String = name.as_str().into();
	let pronouns = get_pronouns_from_db(&*conn, &useful_name);
	Json(useful_name)
}

fn main() {
    rocket::ignite()
             .attach(PeopleDBConn::fairing())
             .mount("/", routes![index, hello]).launch();
}
