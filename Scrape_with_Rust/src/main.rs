use reqwest::Client;
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().gzip(true).build()?;

    let response = client
        .get("https://www.amazon.in/s?k=fossil&page=2")
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .header(reqwest::header::ACCEPT_LANGUAGE, "en-US,en;q=0.5")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());

    let body = response.text().await?;

    println!("Body preview: {}", &body[..500.min(body.len())]);

    let document = Html::parse_document(&body);

    let prod_sel = Selector::parse("div[data-component-type='s-search-result']").unwrap();
    let title_sel = Selector::parse("h2 span.a-text-normal").unwrap();
    let price_sel = Selector::parse("span.a-price-whole").unwrap();
    let rating_sel = Selector::parse("span.a-icon-alt").unwrap();
    let review_count_sel = Selector::parse("span.a-size-base").unwrap();

    let html_products = document.select(&prod_sel);
    let mut rating_vec: Vec<String> = Vec::new();
    let mut title_vec: Vec<String> = Vec::new();
    let mut price_vec: Vec<String> = Vec::new();
    let mut review_count_vec: Vec<String> = Vec::new();

    for product in html_products {
        if let Some(title_element) = product.select(&title_sel).next() {
            let title = title_element.text().collect::<String>();
            if !title.is_empty() {
                title_vec.push(title);
            }
        }

        if let Some(price_element) = product.select(&price_sel).next() {
            let price = price_element.text().collect::<String>();
            if !price.is_empty() {
                price_vec.push(price);
            }
        }

        if let Some(rating_element) = product.select(&rating_sel).next() {
            let rating = rating_element.text().collect::<String>();
            if !rating.is_empty() {
                rating_vec.push(rating);
            }
        }

        if let Some(revc_element) = product.select(&review_count_sel).next() {
            let revc = revc_element.text().collect::<String>();
            if !revc.is_empty() {
                review_count_vec.push(revc);
            }
        }
    }
    for i in 0..title_vec.len() {
        println!("Title: {}", title_vec[i]);
        println!("Price: {}", price_vec[i]);
        println!("Rating: {}", rating_vec[i]);
        println!("Review Count: {}", review_count_vec[i]);
        println!();
    }

    Ok(())
}
