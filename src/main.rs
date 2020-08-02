#![feature(proc_macro_hygiene, decl_macro)]


use rocket::http::RawStr;
use serde::Serialize;
use rocket_contrib::json::Json;


#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct Person {
	name: String
}

impl Person {
	fn new(name: &str) -> Person {
		Person { name: name.into() }
	}
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> Json<Person> {
	let person = Person::new(name.as_str());
	Json(person)
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}
