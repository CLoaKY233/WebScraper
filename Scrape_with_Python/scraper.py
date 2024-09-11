import random
import time
import pandas as pd # type:ignore
import requests # type:ignore
from bs4 import BeautifulSoup # type:ignore

def scrape_amazon(search_term, num_pages=1):
    base_url = "https://www.amazon.in/s?k="
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
    }

    all_products = []

    for page in range(1, num_pages + 1):
        url = f"{base_url}{search_term}&page={page}"
        response = requests.get(url, headers=headers)

        # Check if request was successful
        if response.status_code != 200:
            print(f"Failed to retrieve page {page}. Status code: {response.status_code}")
            continue

        soup = BeautifulSoup(response.content, 'html.parser')

        # Find all product divs
        products = soup.find_all('div', {'data-component-type': 's-search-result'})

        for product in products:
            # Extract product details
            title = product.find('h2', {'class': 'a-size-mini a-spacing-none a-color-base s-line-clamp-2'})
            price = product.find('span', {'class': 'a-price-whole'})
            rating = product.find('span', {'class': 'a-icon-alt'})
            reviews = product.find('span', {'class': 'a-size-base'})

            # Create product data dictionary
            product_data = {
                'Title': title.text.strip() if title else 'N/A',
                'Price': price.text.strip() if price else 'N/A',
                'Rating': rating.text.split()[0] if rating else 'N/A',
                'Reviews': reviews.text.strip() if reviews else 'N/A'
            }

            all_products.append(product_data)

        # Delay between requests
        # time.sleep(random.uniform(1, 3))

    return all_products

def main():
    # Get user input
    search_term = input("Enter the product you want to search for on Amazon: ")
    num_pages = int(input("Enter the number of pages to scrape: "))

    print(f"Scraping data for '{search_term}' from {num_pages} pages...")

    # Start timing
    start_time = time.time()

    products = scrape_amazon(search_term, num_pages)

    if products:
        # Create DataFrame and save to CSV
        df = pd.DataFrame(products)
        csv_filename = f"amazon_{search_term.replace(' ', '_')}.csv"
        df.to_csv(csv_filename, index=False)

        # End timing
        end_time = time.time()

        # Calculate duration
        duration = end_time - start_time

        print(f"Data has been scraped and saved to {csv_filename}")
        print(f"Time taken: {duration:.2f} seconds")
    else:
        print("No products found or there was an error during scraping.")

if __name__ == "__main__":
    main()
