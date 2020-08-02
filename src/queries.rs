use super::schema;
use super::models;

pub fn get_pronouns_from_db(conn: &diesel::mysql::MysqlConnection, name: &String) -> i32 {
        use schema::people::dsl::*;

        let results = people.filter(name.eq(name))
                .limit(1)
                .load::<models::Person>(conn)
                .expect("Woops");
        println!("pretending a little {:?}", results);
        5
}
