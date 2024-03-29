
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
use feed_rs::parser;
use feed_rs::model::*;

use database_connector::models::Article;
use crate::webclient::*;
 


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
    pub complete: bool,
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
            complete: false,
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
                let mut item_complete: bool = true;
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
                    /*if body.is_none() && !self.full_text_tag.is_none() {
                        log::error!("No body in article.");
                        self.success = false;
                        return self;
                    }*/
                    let date: Option<SystemTime>;
                    if item.date.is_none() {
                        date = Option::Some(SystemTime::now());
                    }
                    else {
                        date = item.date;
                    }
                    let author = Option::Some(item.authors.first().unwrap().name.clone());
                    let link = item.link.first().unwrap().href.clone();
                    
                    if title.clone().is_empty() || body.clone().is_none() {
                        item_complete = false;
                    }

                    let article = Article {
                        guid,
                        targert_guid: self.guid,
                        title,
                        summary,
                        body,
                        date,
                        author,
                        link
                    };

                    self.articles.push(article);
                }
                if item_complete {
                    self.complete = true;
                }
                drop(item_complete);
            }
        }
        else {
            log::error!("rss for target '{}' returned with status {}. Message: {}",
                            self.guid, response.status.clone(), response.body);
            
            self.success = false;
        }
        return self;
    }

    pub async fn crawl_items(self) -> Self {
        panic!("Not implemented!");
        /*
        for article in self.articles {
            if article.title == "" {
                article.title = "TODO";
            }
            if article.body == "" {
                article.body = "TODO";
            }
            if article.summary == "" {
                article.summary = "TODO";
            }
            if article.date == "" {
                article.date = "TODO";
            }
            if article.author == "" {
                article.author = "TODO";
            }
        }
        */
        return self;
    }
}