use uuid::Uuid;
use std::time::SystemTime;

#[derive(Queryable, Clone)]
pub struct Target {
    pub guid: Uuid,
    pub name: String,
    pub url: String,
    pub active: bool,
    pub interval: i32,
    pub last_crawl: Option<SystemTime>,
    pub creation_time: Option<SystemTime>,
    pub dns: String,
    pub comments: Option<String>,
    pub logs: Option<String>,
    pub fulltext_tag: Option<String>
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        return self.guid == other.guid &&
        self.name == other.name &&
        self.url == other.url &&
        self.interval == other.interval &&
        self.last_crawl == other.last_crawl &&
        self.creation_time == other.creation_time
    }
}

// to insert targets, should follow https://diesel.rs/guides/getting-started
