use futures::future::join_all;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use std::time::Instant;
use tokio;

// Struct to hold product information
#[derive(Debug, Serialize)]
struct Product {
    title: String,
    price: f64,
    rating: f32,
    review_count: u32,
}

// Struct to hold CSS selectors
struct Selectors {
    prod: Selector,
    title: Selector,
    price: Selector,
    rating: Selector,
    review_count: Selector,
}

// Function to fetch and parse a single page
async fn fetch_page(
    thread_num: &u32,
    url: &str,
    client: Client,
    sel: Arc<Selectors>,
) -> Vec<Product> {
    let mut products = Vec::new();

    // Fetch the page
    let response = match client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .header(reqwest::header::ACCEPT_LANGUAGE, "en-US,en;q=0.5")
        .send()
        .await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Error fetching page: {}", e);
                return products;
            }
        };

    // println!("Status: {}", response.status());
    // not printing for spam

    // Get the HTML content
    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Error reading response body: {}", e);
            return products;
        }
    };

    let document = Html::parse_document(&body);

    // Extract product information
    for product in document.select(&sel.prod) {
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
    println!("thread {} finished", thread_num);
    products
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(Client::builder().gzip(true).build()?);

    // Get user input
    let product_name = read_input_str("Enter the product name: ");
    let total_pages: u32 = read_input_str("Enter the total pages: ").parse()?;
    let print_output = read_input_bool("Do you want to print the output (y/n): ");

    let start = Instant::now();
    let mut all_products = Vec::new();

    // Initialize selectors
    let selectors = Arc::new(Selectors {
        prod: Selector::parse("div[data-component-type='s-search-result']").unwrap(),
        title: Selector::parse("h2 span.a-text-normal").unwrap(),
        price: Selector::parse("span.a-price-whole").unwrap(),
        rating: Selector::parse("span.a-icon-alt").unwrap(),
        review_count: Selector::parse("span.a-size-base").unwrap(),
    });

    // Create tasks for each page
    let mut tasks = Vec::with_capacity(total_pages as usize);
    println!("Fetching data from Amazon.in");
    println!("creating {} threads for {} pages", total_pages, total_pages);
    for i in 1..=total_pages {
        let url = format!("https://www.amazon.in/s?k={}&page={}", product_name, i);
        let client = Arc::clone(&client);
        let selectors = Arc::clone(&selectors);

        let task =
            tokio::spawn(async move { fetch_page(&i, &url, (*client).clone(), selectors).await });
        tasks.push(task);
    }

    // Execute all tasks concurrently
    let results = join_all(tasks).await;
    for result in results {
        match result {
            Ok(products) => all_products.extend(products),
            Err(e) => eprintln!("Error in task: {:?}", e),
        }
    }

    // Print output if requested
    if print_output {
        for product in &all_products {
            println!("Title: {}", product.title);
            println!("Price: {:.2}", product.price);
            println!("Rating: {:.1}", product.rating);
            println!("Review Count: {}", product.review_count);
            println!();
        }
    }

    // Write data to CSV file
    println!("Writing data to file");
    let mut prod_name = product_name.replace(" ", "_");
    prod_name = format!("{}.csv", prod_name);
    let file = File::create(prod_name)?;
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_writer(file);

    for product in &all_products {
        wtr.serialize(product)?;
    }

    wtr.flush()?;
    println!("Data written to file");

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}

// Helper function to read string input
fn read_input_str(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Helper function to read boolean input
fn read_input_bool(prompt: &str) -> bool {
    loop {
        let input = read_input_str(prompt).to_lowercase();
        match input.as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}
