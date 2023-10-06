use reqwest;
use select::{self, document::Document, predicate::Class};
use std::{fs, thread::JoinHandle};

#[tokio::main]
async fn main() {
    let file = std::env::args().nth(1).expect("Expected one argument");
    let mut url_vec: Vec<String> = Vec::new();
    match fs::read_to_string(file) {
        Ok(all_urls) => {
            let lines = all_urls.lines();
            for line in lines {
                url_vec.push(line.to_string());
            }
        }
        Err(msg) => panic!("{}", msg),
    }

    let mut joins: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for url in url_vec {
        joins.push(tokio::spawn(async move {
            match get_date_and_title(url).await {
                Err(e) => println!("ERROR: {}", e),
                _ => {}
            }
        }))
    }

    for task in joins {
        task.await.expect("ERROR");
    }
}

async fn get_date_and_title(url: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello from Thread");

    let html = reqwest::get(url).await?.text().await?;

    let document = Document::from(html.as_str());
    let title = document
        .find(Class("entry-title"))
        .nth(0)
        .expect("No Title given")
        .text();
    let date = document
        .find(Class("meta-date"))
        .nth(0)
        .expect("No date given")
        .text();

    println!("The article \"{}\" was published on [{}]", title, date);
    Ok(())
}
