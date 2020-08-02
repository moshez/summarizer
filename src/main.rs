#![feature(proc_macro_hygiene, decl_macro)]


use rocket::http::RawStr;
use serde::Serialize;
use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel;


#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;


#[database("people")]
struct PeopleDBConn(diesel::mysql::MysqlConnection);


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct Person {
	name: String,
}

fn get_pronouns_from_db(conn: &diesel::mysql::MysqlConnection, name: &String) -> Pronouns {
	println!("pretending a little");
	Pronouns { subject: "they".into(), object: "them".into(), posessive: "they".into() }
}

#[derive(Serialize)]
struct Pronouns {
	subject: String,
	object: String,
	posessive: String,
}

impl Person {
	fn new(name: &str) -> Person {
		Person { name: name.into() }
	}
}

#[get("/hello/<name>")]
fn hello(conn: PeopleDBConn, name: &RawStr) -> Json<Person> {
	let person = Person::new(name.as_str());
	let pronouns = get_pronouns_from_db(&*conn, &person.name);
	Json(person)
}

fn main() {
    rocket::ignite()
             .attach(PeopleDBConn::fairing())
             .mount("/", routes![index, hello]).launch();
}
