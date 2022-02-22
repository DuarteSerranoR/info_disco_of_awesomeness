use std::str;
use hyper::Client;

#[derive(Clone)]
pub struct WebClient {
    url: String,
    pub success: bool,
    pub status: u16,
    pub body: String,
    result: Option<Result<(String, u16), (String, u16)>>
}

impl WebClient {
    pub fn new() -> Self {
        return Self {
            url: String::new(),
            success: false,
            status: 0,
            body: String::new(),
            result: Option::None
        }
    }

    pub async fn get(mut self, url: String) -> Self {

        self.url = url;
        
        self = self.request().await;

        self.success = self.result.clone().unwrap().is_ok();
        self.body = self.result.clone().unwrap().unwrap().0;
        self.status = self.result.clone().unwrap().unwrap().1;

        return self;
    }

    async fn request(mut self) -> Self {
    
        let url = self.url.parse::<hyper::Uri>().unwrap();
        self.result = Option::Some(self.clone().fetch_url(url).await);
        return self;
    }

    async fn fetch_url(self, url: hyper::Uri) -> Result<(String, u16), (String, u16)> {
        drop(self);

        let client = Client::new();

        let response = match client.get(url).await {
            Ok(response) => response,
            Err(error) => return Result::Err((String::from(error.to_string()), 0)) // TODO - http vs https
        };

        let status = response.status().as_u16();
        
        let response_bytes = match hyper::body::to_bytes(response).await {
            Ok(response_bytes) => response_bytes,
            Err(error) => return Result::Err((String::from(error.to_string()), status))
        };
    
        return match str::from_utf8(&response_bytes) { // TODO - implement other encodings like ISO 8859 (...)
            Ok(response_str) => Result::Ok((String::from(response_str), status)),
            Err(error) => Result::Err((String::from(error.to_string()), status))
        };

        //  Alternative to later test, this way, we go directly to getting the request string:
        //let mut body = String::new();
        //result.read_to_string(&mut body).unwrap();
        
    }
}

