use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::time::Instant;
use tokio;

#[derive(Debug, Serialize)]
struct Product {
    title: String,
    price: f64,
    rating: f32,
    review_count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().gzip(true).build()?;

    //user input product

    let product_name = readinputstr("Enter the product name: ");

    let total_pages = readinputstr("Enter the total pages: ");

    let page_num: u32 = total_pages.parse().expect("Please enter a valid number");

    let print_output = readinputbool("Do you want to print the output (y/n): ");

    let start = Instant::now();

    let mut products: Vec<Product> = Vec::new();

    for i in 1..(page_num + 1) {
        let url = format!("https://www.amazon.in/s?k={}&page={}", product_name, i);
        println!("Fetching data from: {}", url);
        let response = client
            .get(&url)
            .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .header(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
            .header(reqwest::header::ACCEPT_LANGUAGE, "en-US,en;q=0.5")
            .send()
            .await?;

        println!("Status: {}", response.status());
        // println!("Headers: {:#?}", response.headers());

        let body = response.text().await?;

        // println!("Body preview: {}", &body[..500.min(body.len())]);

        let document = Html::parse_document(&body);

        let prod_sel = Selector::parse("div[data-component-type='s-search-result']").unwrap();
        let title_sel = Selector::parse("h2 span.a-text-normal").unwrap();
        let price_sel = Selector::parse("span.a-price-whole").unwrap();
        let rating_sel = Selector::parse("span.a-icon-alt").unwrap();
        let review_count_sel = Selector::parse("span.a-size-base").unwrap();

        let html_products = document.select(&prod_sel);
        // let mut rating_vec: Vec<String> = Vec::new();
        // let mut title_vec: Vec<String> = Vec::new();
        // let mut price_vec: Vec<String> = Vec::new();
        // let mut review_count_vec: Vec<String> = Vec::new();

        for product in html_products {
            let title = product
                .select(&title_sel)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_else(|| "N/A".to_string());

            let price = product
                .select(&price_sel)
                .next()
                .and_then(|el| {
                    el.text()
                        .collect::<String>()
                        .replace(",", "")
                        .parse::<f64>()
                        .ok()
                })
                .unwrap_or(0.0);

            let rating = product
                .select(&rating_sel)
                .next()
                .and_then(|el| {
                    el.text()
                        .collect::<String>()
                        .split_whitespace()
                        .next()
                        .and_then(|s| s.parse::<f32>().ok())
                })
                .unwrap_or(0.0);

            let review_count = product
                .select(&review_count_sel)
                .next()
                .and_then(|el| {
                    el.text()
                        .collect::<String>()
                        .replace(",", "")
                        .parse::<u32>()
                        .ok()
                })
                .unwrap_or(0);

            products.push(Product {
                title,
                price,
                rating,
                review_count,
            });
        }
        if print_output {
            for product in &products {
                println!("Title: {}", product.title);
                println!("Price: {:.2}", product.price);
                println!("Rating: {:.1}", product.rating);
                println!("Review Count: {}", product.review_count);
                println!();
            }
        }
    }
    let file = File::create("OutDir/output.csv")?;
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_writer(file);
    for product in &products {
        wtr.serialize(product)?;
    }
    match wtr.flush() {
        Ok(_) => {
            println!("Data written to file");
            let duration = start.elapsed();
            println!("Time elapsed: {:?}", duration);
        }
        Err(e) => println!("Error writing to file: {}", e),
    };

    Ok(())
}

fn readinputstr(inputline: &str) -> String {
    print!("{}", inputline);
    match stdout().flush() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }; // Flush the buffer
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn readinputbool(inputline: &str) -> bool {
    print!("{}", inputline);
    match stdout().flush() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }; // Flush the buffer
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    if input.trim().to_string() == "y" {
        return true;
    } else if input.trim().to_string() == "n" {
        return false;
    } else {
        readinputbool(inputline);
    }
    false
}
