extern crate diesel;

use crate::models::Target;
use self::diesel::prelude::*;

pub fn get_active_targets() -> Vec<Target> {
    use crate::schema::targets::dsl::*;
    use crate::establish_connection;

    let connection: PgConnection = establish_connection();
    let results = targets
        .filter(active.eq(true))
        .load::<Target>(&connection)
        .expect("Error loading targets");
    
    return results;
}
