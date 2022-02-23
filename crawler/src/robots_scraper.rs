use crate::webclient::WebClient;
use database_connector::models::Target;
use robotstxt::{DefaultMatcher, matcher::{LongestMatchRobotsMatchStrategy, RobotsMatcher}};


pub struct Robots<'a> {
    pub robots_url: String,
    pub matcher: Option<RobotsMatcher<'a, LongestMatchRobotsMatchStrategy>>,
    pub body: String,
    //pub crawl_delay: Option<i32>,
    //pub disallowed_vec: Vec<String>,
    //pub allowed_vec: Vec<String>
    pub success: bool,
    pub should_crawl: bool
}

impl Robots<'static> {

    pub async fn load_robots(mut self, target: Target, dns: String) -> Robots<'static> {
        self.robots_url = format!("https://{}/robots.txt", dns);
        drop(dns);

        let web_client = WebClient::new();
        let response = web_client.get(self.robots_url.clone()).await;

        if response.success.clone() {
            self.body = response.body.clone();
            self.should_crawl = true;
            self.success = true;
            drop(target);
            drop(response);
        }
        else {
            self.body = String::from("");
            log::error!("Robots.txt for target '{}' returned with status {}. Message: {}", 
                        target.guid, response.status.clone(), response.body);
            drop(target);
            self.success = false;

            // Check if we should crawl at all
            match response.status {
                0 => self.should_crawl = false, // Program's fault
                401 => self.should_crawl = false, // Unauthorized
                402 => self.should_crawl = false, // Payment Required
                403 => self.should_crawl = false, // Forbidden
                _ => self.should_crawl = true
            }
            drop(response);
            return self;
        }
        
        //self.robots_url.as_ref().unwrap().as_str()

        // TODO -> make my own implementation for crawl_interval and other necessities
        let matcher: RobotsMatcher<LongestMatchRobotsMatchStrategy> = DefaultMatcher::default();
        self.matcher = Option::Some(matcher);
        return self;
    }
    
    // usage -> assert_eq!(false, matcher.one_agent_allowed_by_robots(body, "user-agent", url to match));
    pub fn check_url(self, url: String) -> bool {
        let mut user_agents_vec: Vec<&str> = Vec::new();
        user_agents_vec.push("Du-Bot");
        user_agents_vec.push("*");
        return self.matcher.unwrap().allowed_by_robots(self.body.as_ref(), user_agents_vec, url.as_str());
    }

    /*
    pub fn get_interval() -> i32 {
        return self.matcher.unwrap().;
    }
    */
}