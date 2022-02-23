extern crate hyper;
use std::str;
use hyper::Client;
use hyper_tls::HttpsConnector;

#[derive(Clone)]
pub struct WebClient {
    url: String,
    pub success: bool,
    pub status: u16,
    pub body: String,
    result: Result<(String, u16), (String, u16)>
}

impl WebClient {
    pub fn new() -> Self {
        return Self {
            url: String::new(),
            success: false,
            status: 0,
            body: String::new(),
            result: Err((String::from("Request not made"), 0))
        }
    }

    pub async fn get(mut self, url: String) -> Self {

        self.url = url;
        
        self = self.request().await;

        return self;
    }

    async fn request(mut self) -> Self {
    
        let url = self.url.parse::<hyper::Uri>().unwrap();
        self.result = fetch_url(url).await;

        self.success = self.clone().result.is_ok();
        self.body = self.clone().result.unwrap().0;
        self.status = self.clone().result.unwrap().1;
        
        //log::info!("{}", format!("success: {}, status {}, body {}", self.clone().success, self.clone().status.to_string(), self.clone().body));

        return self;
    }
}

async fn fetch_url(url: hyper::Uri) -> Result<(String, u16), (String, u16)> {

    let https = HttpsConnector::new();
    let client = Client::builder()
                                                    .build::<_, hyper::Body>(https);

    let response = match client.get(url).await {
        Ok(response) => response,
        Err(error) => return Result::Err((String::from(error.to_string()), 0)) // TODO - http vs https
    };

    let status = response.status().as_u16();
    
    let response_bytes = match hyper::body::to_bytes(response).await {
        Ok(response_bytes) => response_bytes,
        Err(error) => return Result::Err((String::from(error.to_string()), status))
    };

    match str::from_utf8(&response_bytes) { // TODO - implement other encodings like ISO 8859 (...)
        Ok(response_str) => return Result::Ok((String::from(response_str), status)),
        Err(error) => return Result::Err((String::from(error.to_string()), status))
    };

    //  Alternative to later test, this way, we go directly to getting the request string:
    //let mut body = String::new();
    //result.read_to_string(&mut body).unwrap();

    // Other options to implement -> https://docs.rs/hyper-tls/latest/hyper_tls/
}