use reqwest;
use select::{self, document::Document, predicate::Class};
use std::fs;

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
        match task.await {
            _ => {}
        }
    }
}

async fn get_date_and_title(url: String) -> Result<(), String> {
    let html = reqwest::get(&url)
        .await
        .expect(format!("ERROR: GET {}", url).as_str())
        .text()
        .await
        .expect(format!("ERROR: Parse result").as_str());

    let document = Document::from(html.as_str());

    let title = match document.find(Class("entry-title")).nth(0) {
        Some(str) => str.text(),
        None => return Err(format!("Article not valid \"{}\" - No title", url)),
    };

    let date = match document.find(Class("meta-date")).nth(0) {
        Some(str) => str.text(),
        None => return Err(format!("Article not valid \"{}\" - No date", url)),
    };

    println!();
    println!("{:20}", "-");
    println!("The article \"{}\" was published on [{}]", title, date);
    Ok(())
}
