extern crate database_connector;
extern crate diesel;

use self::database_connector::*;
use self::models::*;
use self::diesel::prelude::*;
// use diesel::pg::types::sql_types::Uuid;

fn main() {
    use database_connector::schema::targets::dsl::*;

    let connection: PgConnection = establish_connection();
    let results = targets
        .limit(5)
        .load::<Target>(&connection)
        .expect("Error loading targets");

    println!("Displaying {} targets", results.len());
    for target in results {
        println!("{}", target.name);
        println!("----------\n");
        println!("{}", target.url);
    }
}