use uuid::Uuid;

use database_connector::models::*;
use database_connector::functions::*;
use logger;
use crate::logger::setup_logger;

use crawler::robots_scraper::*;

fn main() {
    setup_logger().expect("");
    let targets_vec = get_active_targets();

    // Tests from db
    let test_target: Target = targets_vec.into_iter()
        .find(|t| t.guid == Uuid::parse_str("9cd64569-7861-4cf5-9e9a-4b22670ff136").unwrap())
        .unwrap();

    /* HardcodedTests
    let test_target: Target = Target {
        guid: Uuid::parse_str("9cd64569-7861-4cf5-9e9a-4b22670ff136").unwrap(),
        name: String::from("Microsoft Research"),
        url: String::from("https://www.microsoft.com/en-us/research/feed/"),
        active: true,
        interval: 120,
        last_crawl: Option::None,
        creation_time: Option::Some(SystemTime::now())
    };
    */

    log::info!("Testing: {}", test_target.name);
    log::info!("Crawling Started");

    // TODO
    let robots: Robots = Robots {
        robots_url: None,
        disalowed_vec: Vec::new(),
        crawl_delay: None
    };
    robots.load_robots(test_target.url);
}