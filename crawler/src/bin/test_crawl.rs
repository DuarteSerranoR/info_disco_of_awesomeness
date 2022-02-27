use uuid::Uuid;
use std::time::Duration;
use std::thread::sleep;

use database_connector::models::*;
use database_connector::functions::*;
use logger;
use crate::logger::setup_logger;

use crawler::robots_scraper::*;
use crawler::rss_scraper::*;

#[tokio::main(flavor = "current_thread")] // https://docs.rs/tokio/1.4.0/tokio/attr.main.html
async fn main() {
    setup_logger().expect("");
    let targets_vec = get_active_targets();

    // Tests from db
    let test_target: Target = targets_vec.into_iter()
        //.find(|t| t.guid == Uuid::parse_str("9cd64569-7861-4cf5-9e9a-4b22670ff136").unwrap())
        //.find(|t| t.guid == Uuid::parse_str("298e6222-eb8a-4b9d-aa1f-c64832a4b50f").unwrap())
        .find(|t| t.guid == Uuid::parse_str("2d294334-31d0-4670-ad2b-34d2b413a183").unwrap())
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
    drop(time);

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
        let mut rss_scraper = Rss::new(test_target.guid, test_target.url, /*Option::None,*/ test_target.fulltext_tag);
        rss_scraper = rss_scraper.crawl_rss().await;
        
        let articles = rss_scraper.clone().articles;

        drop(rss_scraper);
        
        /*
        for article in rss_scraper.articles {
            if robots.check_url(article) {
                rss_scraper.crawl_item(article);
            }
        }
        */

        let mut num_feeds = 0;
        for article in articles {
            let result = insert_article(article.clone());
            if result.is_err() {
                log::error!("Failed to insert article '{}'. Exception: {}",
                            article.guid, result.err().unwrap());
            }
            else {
                num_feeds += 1;
            }
            drop(article);
        }

        log::info!("Target '{}' inserted {} feeds.", test_target.name, num_feeds)

        //drop(test_target);
        
        // TODO - target logs after crawl (put exceptions, ......)

        //drop(articles);
        // If product, then to database, otherwise (here) print the formated results
    }
}