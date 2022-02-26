
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// RSS Scraper
/// 
/// The purpose of this Module is to crawl into an rss feed, and throught its channels,
/// and items generate articles, so they can later be sent into the database and used.
/// This Scraper takes into account the page's robots.txt needs 
/// (sent through the construtor) and follows the apointed rules. It also allows you
/// to specify full text tags, so that you don't need to later crawl into the items
/// and get the Articles body through their links, making it faster and safer.
/// This module uses the feed-rs crate to parse the rss feeds.
/// 
/// Usage:
///     1-> 
/// 
/// Results:
///     With your 
/// 
///             guid: Uuid -> 
/// 
/// 
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::time::SystemTime;

use uuid::Uuid;

use crate::webclient::*;
use feed_rs::parser;
use feed_rs::model::*;
 


// TODO - implement this Article object inside the database_connector's codefirst 
//      so it can be implemented in the database
#[derive(Clone)]
pub struct Article {
    pub guid: String,
    pub title: String,
    pub summary: Option<String>,
    pub body: Option<String>,
    pub date: SystemTime,
    pub author: String,
    pub link: String
}

///////////////////////////////////////////////////////////////////////////////
/// Crawled Item Object
/// 
/// Represents the Items present in the rss's channels, these are
/// later converted into Articles, so they can be sent to the database
/// and used from then on.
///////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Item {
    pub guid: String,
    pub title: Option<Text>,
    pub summary: Option<Text>,
    pub body: Option<String>,
    pub date: Option<SystemTime>,
    pub authors: Vec<Person>,
    pub link: Vec<Link>   // TODO - implement this object in a separate module "item_crawler"
                          //        so it can later be crawled separately (if there is no 
                          //        full_text_tag).
}

///////////////////////////////////////////////////////////////////////////////
/// Channel Object
///////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Channel {
    pub items: Vec<Item>,
}

impl Channel {
    pub fn new() -> Self { Self { items: Vec::new() } }
}

///////////////////////////////////////////////////////////////////////////////
/// Rss Object
/// 
/// Main Object in the rss_scraper module.
/// Used to crawl items from an rss feed, uses the provided url
/// to crawl its channel's items and convert them into readable
/// Articles, returning those articles so they can be saved 
/// into the database.
///////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Rss {
    guid: Uuid,
    url: String,
    pub channels: Vec<Channel>,
    pub articles: Vec<Article>,
    //robots_txt: Option<Robots<'a>>,
    success: bool,
    full_text_tag: Option<String>
}

///////////////////////////////////////////////////////////////////////////////
/// RSS Object's functions
/// 
/// Used to crawl your rss and get the wanted items in a
/// neat and simple way. ;)
///////////////////////////////////////////////////////////////////////////////
impl Rss {

    ///////////////////////////////////////////////////////////////////////////////
    /// Constructors used to create a new RSS
    ///////////////////////////////////////////////////////////////////////////////
    pub fn new(guid: Uuid, url: String/*, robots_txt: Option<Robots<'static>>*/, full_text_tag: Option<String>) -> Self {
        return Self {
            guid,
            url,
            channels: Vec::new(),
            articles: Vec::new(),
            //robots_txt,
            success: false,
            full_text_tag
        };
    }

    pub async fn crawl_rss(mut self) -> Self {
        let web_client = WebClient::new();
        let response = web_client.get(self.url.clone()).await;
        if response.success.clone() {
            self.success = true;
            let xml = response.body;
            let feed = parser::parse(xml.as_bytes()).unwrap();

            let mut channel = Channel {
                items: Vec::new()
            };

            // Store items into a vector
            for entrie in feed.entries {
                let item = Item {
                    guid: entrie.id,
                    title: entrie.title,
                    summary: entrie.summary,
                    body: entrie.content.unwrap().body,
                    date: Option::None,
                    authors: entrie.authors,
                    link: entrie.links
                };

                if item.body.is_none() && !self.full_text_tag.is_none() {
                    // TODO - use xPath
                }

                channel.items.push(item);
            }
            self.channels.push(channel);

            // Transform the items into Articles
            for channel in self.channels.clone() {
                for item in channel.items {

                    let guid = item.guid;
                    let title = item.title.unwrap().content;
                    let mut summary: Option<String> = Option::None;
                    if item.summary.is_none() {
                        log::warn!("No summary")
                    }
                    else {
                        summary = Option::Some(item.summary.unwrap().content);                        
                    }

                    let body = item.body;
                    if body.is_none() && !self.full_text_tag.is_none() {
                        log::error!("No body in article.");
                        self.success = false;
                        return self;
                    }
                    let date: SystemTime;
                    if item.date.is_none() {
                        date = SystemTime::now();
                    }
                    else {
                        date = item.date.unwrap();
                    }
                    let author = item.authors.first().unwrap().name.clone();
                    let link = item.link.first().unwrap().href.clone();

                    let article = Article {
                        guid,
                        title,
                        summary,
                        body,
                        date,
                        author,
                        link
                    };

                    self.articles.push(article);
                }
            }
        }
        else {
            log::error!("rss for target '{}' returned with status {}. Message: {}",
                            self.guid, response.status.clone(), response.body);
            
            self.success = false;
        }
        return self;
    }
}