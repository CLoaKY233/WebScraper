use futures::future::join_all;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use std::time::Instant;
use tokio;

#[derive(Debug, Serialize)]
struct Product {
    title: String,
    price: f64,
    rating: f32,
    review_count: u32,
}

struct Selectors {
    prod: Selector,
    title: Selector,
    price: Selector,
    rating: Selector,
    review_count: Selector,
}

async fn fetch_page(url: &str, client: Client, sel: Arc<Selectors>) -> Vec<Product> {
    let mut products: Vec<Product> = Vec::new();
    let response = match client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .header(reqwest::header::ACCEPT_LANGUAGE, "en-US,en;q=0.5")
        .send()
        .await{
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                return products;
            }
        };

    println!("Status: {}", response.status());
    // println!("Headers: {:#?}", response.headers());

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("Error: {}", e);
            return products;
        }
    };

    let document = Html::parse_document(&body);

    let html_products = document.select(&sel.prod);

    for product in html_products {
        let title = product
            .select(&sel.title)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_else(|| "N/A".to_string());

        let price = product
            .select(&sel.price)
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
            .select(&sel.rating)
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
            .select(&sel.review_count)
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
    return products;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(Client::builder().gzip(true).build()?);
    //user input product
    let product_name = readinputstr("Enter the product name: ");
    let total_pages = readinputstr("Enter the total pages: ");
    let print_output = readinputbool("Do you want to print the output (y/n): ");
    let start = Instant::now();
    let mut all_products: Vec<Product> = Vec::new();
    let selectors = Arc::new(Selectors {
        prod: Selector::parse("div[data-component-type='s-search-result']").unwrap(),
        title: Selector::parse("h2 span.a-text-normal").unwrap(),
        price: Selector::parse("span.a-price-whole").unwrap(),
        rating: Selector::parse("span.a-icon-alt").unwrap(),
        review_count: Selector::parse("span.a-size-base").unwrap(),
    });
    let page_num: u32 = total_pages.parse().expect("Please enter a valid number");
    let mut tasks = Vec::with_capacity(page_num as usize);
    for i in 1..=page_num {
        let url = format!("https://www.amazon.in/s?k={}&page={}", product_name, i);
        println!("Fetching data from: {}", url);

        let client = Arc::clone(&client);
        let selectors = Arc::clone(&selectors);

        let task =
            tokio::spawn(async move { fetch_page(&url, (*client).clone(), selectors).await });
        tasks.push(task);
    }

    let results = join_all(tasks).await;
    for result in results {
        match result {
            Ok(products) => all_products.extend(products),
            Err(e) => println!("Error in task: {:?}", e),
        }
    }

    if print_output {
        for product in &all_products {
            println!("Title: {}", product.title);
            println!("Price: {:.2}", product.price);
            println!("Rating: {:.1}", product.rating);
            println!("Review Count: {}", product.review_count);
            println!();
        }
    }

    let file = File::create("OutDir/output.csv")?;
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_writer(file);
    for product in &all_products {
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
