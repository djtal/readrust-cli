extern crate clap;

use clap::App;

fn main() {
    let app = App::new("readrust")
        .version("0.1")
        .author("Dj T@l")
        .about("Reads rust.net")
        .args_from_usage("-n, --number=[NUMBER] 'Only prinnt the NUMBER of most recent posts'
                         -c, --count            'Show the count of post'");

    let matches = app.get_matches();
}

extern crate reqwest;
pub static URL: &str = "http://readrust.net/rust2018/feed.json";

fn get_feed() -> String {
    let client = reqwest::Client::new();
    let mut request = client.get(URL);

    let mut resp = request.send().unwrap();
    assert!(resp.status().is_success());

    resp.text().unwrap()
}

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct Author {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FeedItem {
    id: String,
    title: String,
    content_text: String,
    url: String,
    date_published: String,
    author: Author,
}

#[derive(Debug, Deserialize, Serialize)]
struct Feed {
    version: String,
    title: String,
    home_page: String,
    feed_url: String,
    author: Author,
    item: Vec<FeedItem>,
}


