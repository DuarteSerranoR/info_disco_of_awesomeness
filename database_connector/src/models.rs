use diesel::pg::types::sql_types::Uuid;
use std::time::SystemTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Target {
    guid: Uuid,
    name: String,
    url: String,
    active: bool,
    interval: i32,
    last_crawl: SystemTime,
    creation_time: SystemTime
}
