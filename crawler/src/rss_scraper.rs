
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// RSS Scraper
/// 
/// The purpose of this Module is to crawl into an rss feed, and throught its channels,
/// and items generate articles, so they can later be sent into the database and used.
/// This Scraper takes into account the page's robots.txt needs 
/// (sent through the construtor) and follows the apointed rules. It also allows you
/// to specify full text tags, so that you don't need to later crawl into the items
/// and get the Articles body through their links, making it faster and safer.
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
 
 

// TODO - implement this Article object inside the database_connector's codefirst 
//      so it can be implemented in the database
pub struct Article {
    pub guid: Uuid,
    pub title: str,
    pub summary: str,
    pub body: str,
    pub date: SystemTime,
    pub author: str,
    pub link: str
}

///////////////////////////////////////////////////////////////////////////////
/// Crawled Item Object
/// 
/// Represents the Items present in the rss's channels, these are
/// later converted into Articles, so they can be sent to the database
/// and used from then on.
///////////////////////////////////////////////////////////////////////////////
pub struct Item {
    pub guid: Option<Uuid>,
    pub title: Option<str>,
    pub summary: Option<str>,
    pub body: Option<str>,
    pub date: Option<SystemTime>,
    pub author: Option<str>,
    pub link: Option<str> // TODO - implement this object in a separate module "item_crawler"
                          //        so it can later be crawled separately (if there is no 
                          //        full_text_tag).
}

///////////////////////////////////////////////////////////////////////////////
/// Channel Object
///////////////////////////////////////////////////////////////////////////////
pub struct Channel {
    pub items: Vec<Item>,
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
pub struct Rss {
    guid: Uuid,
    pub url: str,
    pub channels: Vec<Channel>,
    pub articles: Vec<Article>,
    robots_txt: Option<Robots>,
    full_text_tag: Option<str>
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
    /// 
    /// These apply an overload which states that you don't need
    /// to wrap your the optional Robots or full_text_tag into the
    /// Object creation, they do it for you if null.
    ///////////////////////////////////////////////////////////////////////////////
    pub fn new(guid: Uuid, url: str, robots_txt: Robots, full_text_tag: str) -> Self {
        return Self {

        };
    }

    pub fn new(guid: Uuid, url: str, full_text_tag: str) -> Self {
        return Self {

        };
    }

    pub fn new(guid: Uuid, url: str, robots_txt: Robots) -> Self {
        return Self {

        };
    }

    pub fn new(guid: Uuid, url: str) -> Self {
        return Self {

        };
    }
}