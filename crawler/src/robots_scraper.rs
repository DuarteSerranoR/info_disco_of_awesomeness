use robotstxt::{DefaultMatcher, matcher::{LongestMatchRobotsMatchStrategy, RobotsMatcher}};

pub struct Robots<'a> {
    pub robots_url: Option<String>,
    pub matcher: Option<RobotsMatcher<'a, LongestMatchRobotsMatchStrategy>>,
    pub body: String,
    //pub crawl_delay: Option<i32>,
    //pub disallowed_vec: Vec<String>,
    //pub allowed_vec: Vec<String>
}

impl Robots<'_> {

    pub fn load_robots(mut self, _dns: String) -> Self {
        self.robots_url = Option::Some(format!("https://{}/robots.txt", _dns));

        let robots_body = "";
        
        
        //self.robots_url.as_ref().unwrap().as_str()

        // TODO -> make my own implementation for crawl_interval and other necessities
        let matcher: RobotsMatcher<LongestMatchRobotsMatchStrategy> = DefaultMatcher::default();
        self.body = String::from(robots_body);
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