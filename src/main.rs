// This script scrapes the "r/rust" subreddit page on Reddit and prints out the text of all <h2> headings.

use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error as StdError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    // Define the URL of the page to scrape
    let url = "https://www.reddit.com/r/rust/";

    // Create a new reqwest client with a custom User-Agent to avoid a 403 error
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36") // Spoof as Chrome browser
        .build()?;

    // Make a GET request to fetch the HTML content of the page
    let res = client.get(url).send().await;

    // Check if the request is successful
    match res {
        Ok(response) => {
            if response.status().is_success() {
                // If the response is successful, retrieve the response body as text
                let body = response.text().await?;

                // Parse the HTML body using the scraper crate
                let document = Html::parse_document(&body);
                // Define a selector to target <h2> tags
                let selector = Selector::parse("h2").unwrap();  // Parse all <h2> tags

                // Loop through each selected <h2> element and print its text content
                for element in document.select(&selector) {
                    println!("Found heading: {}", element.text().collect::<Vec<_>>().join(""));
                }
            } else {
                // If the request is unsuccessful, print an error message with the status code
                eprintln!("Request failed with status: {}", response.status());
            }
        }
        Err(e) => {
            // If there is a request error, print an error message
            eprintln!("Failed to fetch the page: {}", e);
        }
    }

    // Return Ok indicating the program ran successfully
    Ok(())
}
