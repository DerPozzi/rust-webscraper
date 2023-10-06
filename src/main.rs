use reqwest;
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
}
