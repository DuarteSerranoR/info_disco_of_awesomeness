use uuid::Uuid;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Target {
    pub guid: Uuid,
    pub name: String,
    pub url: String,
    pub active: bool,
    pub interval: i32,
    pub last_crawl: Option<SystemTime>,
    pub creation_time: Option<SystemTime>
}

// to insert targets, should follow https://diesel.rs/guides/getting-started
