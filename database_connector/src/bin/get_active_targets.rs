extern crate database_connector;
extern crate diesel;

use self::database_connector::*;
use self::functions::get_active_targets;

fn main() {

    let results = get_active_targets();

    println!("Displaying {} targets", results.len());
    for target in results {
        println!("{}", target.name);
        println!("----------\n");
        println!("{}", target.url);
    }
}
