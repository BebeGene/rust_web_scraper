# Rust Web Scraper
This is a simple web scraper built with Rust that extracts headlines from a webpage. The scraper fetches the HTML content from a given URL, parses the content, and extracts all headlines (within `<h3>` tags) using the `scraper` crate.

## Features
* Fetches HTML content from a URL.
* Extracts headlines from the webpage.
* Outputs the list of headlines to the terminal.

## Prerequisites
* Install [Rust](https://www.rust-lang.org/)
* Clone repo
* cd web_scraper/src

## Run
* You can modify the web page to be scraped on line 10 in main.rs
* cargo run

## Screenshots
![AltText](https://github.com/BebeGene/rust_web_scraper/blob/master/Screenshots/Headlines.png)
