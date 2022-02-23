use uuid::Uuid;
use std::time::Duration;
use std::thread::sleep;

use database_connector::models::*;
use database_connector::functions::*;
use logger;
use crate::logger::setup_logger;

use crawler::robots_scraper::*;

#[tokio::main(flavor = "current_thread")] // https://docs.rs/tokio/1.4.0/tokio/attr.main.html
async fn main() {
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
        creation_time: Option::Some(SystemTime::now()),
        dns: String::from("www.microsoft.com")
    };
    */

    log::info!("Testing: {}", test_target.name);
    log::info!("Crawling Started");

    // TODO
    let time: Duration = Duration::from_secs(5); //
    sleep(time); //

    // Load Robots.txt
    let robots: Robots = Robots {
        robots_url: String::new(),
        matcher: Option::None,
        body: String::new(),
        //crawl_delay: None
        should_crawl: true,
        success: false
    };
    let robots = robots.load_robots(test_target.clone(), test_target.dns.clone()).await;

    // Crawl
    if robots.check_url(test_target.url.clone()) {
        log::info!("crawl");
        //if scraper.crawl_rss(test_target.fulltext)

        //  foreach article in scraper.articles
        //      if robots.check_url(article)
        //          scraper.crawl_item(article);

        // If product, then to database, otherwise (here) print the formated results
    }
}