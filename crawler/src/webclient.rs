
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Web Client
/// 
/// The purpose of this Object is to allow the application to make http/https requests
/// without having repeated code and in an optimized way.
/// This object uses the hyper library's client and the hyper_tls's HttpConnector.
/// 
/// Usage:
///     1-> Create the WebClient object
///             Ex.: let web_client: WebClient = WebClient::new();
///     
///     2-> Use the get(url: String) request to fetch an url's response
///             In this request, the url String argument represents the url you 
///             want to get a response from
///             Ex.: let response = web_client.get(url);
/// 
/// Results:
///     With your get request, it returns an object with the following structure:
/// 
///             url: String -> [it's private to the WebClient] represents the 
///                             url where the request was made.
/// 
///             success: bool -> boolean that represents the success of the 
///                             request made.
/// 
///             status: u16 -> 16-bit unsigned integer that represents the 
///                             http status code returned by the url's server.
/// 
///             body: String -> String that contains the body returned with 
///                             the request.
/// 
///             result: Result<(String, u16), (String, u16)> -> 
///                             [it's private to the WebClient] represents the 
///                             Result object generated with the request and 
///                             encoding of the request.
/// 
/// 
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////



extern crate hyper;
use std::str;
use hyper::Client;
use hyper::header::{ USER_AGENT, HeaderMap, HeaderValue };
use hyper_tls::HttpsConnector;



///////////////////////////////////////////////////////////////////////////////
/// Web Client Object
///////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct WebClient {
    url: String,
    pub success: bool,
    pub status: u16,
    pub body: String,
    result: Result<(String, u16), (String, u16)>
}

///////////////////////////////////////////////////////////////////////////////
/// Web Client Object's functions
///////////////////////////////////////////////////////////////////////////////
impl WebClient {

    ///////////////////////////////////////////////////////////////////////////////
    /// Constructor used to create a new WebClient instant
    ///////////////////////////////////////////////////////////////////////////////
    pub fn new() -> Self {
        return Self {
            url: String::new(),
            success: false,
            status: 0,
            body: String::new(),
            result: Err((String::from("Request not made"), 0))
        }
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Method used to use the get request in a 
    /// shared way throughout the project
    ///////////////////////////////////////////////////////////////////////////////
    pub async fn get(mut self, url: String) -> Self {

        self.url = url;
        
        self = self.request().await;

        return self;
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Method used to save the request's status
    ///////////////////////////////////////////////////////////////////////////////
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

///////////////////////////////////////////////////////////////////////////////
/// Method used to send the request and get the response
///////////////////////////////////////////////////////////////////////////////
async fn fetch_url(url: hyper::Uri) -> Result<(String, u16), (String, u16)> {

    let https = HttpsConnector::new();
    let mut headers = HeaderMap::new();

    headers.append(USER_AGENT, HeaderValue::from_static("DuBot"));
    //headers.append(HeaderName::from_static("User-Agent"), HeaderValue::from_static("DuBot"));
    //headers.append("".parse().unwrap(), "".parse().unwrap());
    //headers.append(USER_AGENT, "".parse().unwrap()); -> alternative found at 
                                                        // https://docs.rs/hyper/0.13.1/hyper/struct.HeaderMap.html


    //  An alternative to this hyper::Client would be the hyper::Request -> sets the user_agent/header directly
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

    //  Other options to implement -> https://docs.rs/hyper-tls/latest/hyper_tls/
}
