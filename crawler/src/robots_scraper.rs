pub struct Robots {
    pub robots_url: Option<String>,
    pub disalowed_vec: Vec<String>,
    pub crawl_delay: Option<i32>
}

impl Robots {
    pub fn load_robots(self, _dns: String) -> Robots {
        return self;
    }
    
    pub fn check_url(self, _url: String) -> bool {
        return true;
    }
}