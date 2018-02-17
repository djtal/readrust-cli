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
    let feed = get_feed();

    if matches.is_present("count") {
        print_count(&feed);
    } else {
        let iter = feed.items.iter();

        if let Some(string) = matches.value_of("number") {
            let number = string.parse().unwrap();
            print_feed_table(iter.take(number))
        } else {
            print_feed_table(iter)
        }
    }
}

extern crate reqwest;
pub static URL: &str = "http://readrust.net/rust2018/feed.json";

fn get_feed() -> Feed {
    let client = reqwest::Client::new();
    let mut request = client.get(URL);

    let mut resp = request.send().unwrap();
    assert!(resp.status().is_success());

    let json = resp.text().unwrap();

    serde_json::from_str(&json).unwrap()
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
    home_page_url: String,
    feed_url: String,
    author: Author,
    items: Vec<FeedItem>,
}

fn print_count(feed: &Feed) {
    println!("Number of posts : {}", feed.items.len());
}

#[macro_use]
extern crate prettytable;

fn print_feed_table<'feed, I: Iterator<Item = &'feed FeedItem>>(items: I) {
    let mut table = prettytable::Table::new();

    table.add_row(row!["Title", "Author", "Link"]);

    for item in items {
        let title = if item.title.len() >= 50 {
            &item.title[0..49]
        } else {
            &item.title
        };

        table.add_row(row![title, item.author.name, item.url]);
    }

    table.printstd();
}
