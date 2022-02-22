use hyper::Error;
use std::str;
use hyper::Client;

async fn get_str(url: String) -> String {
    
    let response = request(url);

    return String::from("");
}

#[tokio::main]
async fn request(url: String) -> String {

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    let response = fetch_url(url).await;
    return response.unwrap();
}

async fn fetch_url(url: hyper::Uri) -> Result<String, Error> { // TODO -> pass the error to the target to log
    let client = Client::new();

    let res = client.get(url).await?;
    
    let buf = hyper::body::to_bytes(res).await?;

    let result = match str::from_utf8(&buf) {
        Ok(response_str) => String::from(response_str),
        Err(error) => String::from(error.to_string())//Error::from(err))//)err),
    };

    return Result::Ok(result);
}