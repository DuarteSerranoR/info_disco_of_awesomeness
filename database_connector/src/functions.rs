extern crate diesel;

use diesel::result::Error;

use crate::models::Target;
use crate::models::Article;
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

pub fn insert_article(article: Article) -> Result<bool, Error> {
    use crate::schema::articles;
    use crate::establish_connection;

    let connection: PgConnection = establish_connection();

    match diesel::insert_into(articles::table)
                .values(&article)
                .execute(&connection) {
                    Ok(_) => {
                        return Ok(true)
                    }
                    Err(ex) => {
                        return Err(ex)
                    }
                }
}