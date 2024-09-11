# Amazon Product Scraper (Python)

This Python script efficiently scrapes product information from Amazon India based on user input.

## Prerequisites

- Python 3.7+
- [uv](https://github.com/astral-sh/uv) (recommended) or pip

## Installation
1. Clone and navigate to the repository:
   ```
   git clone https://github.com/CLoaKY233/WebScraper.git
   cd Scrape_with_rust
   ```

2. Install uv (if not already installed):
   ```
   pip install uv
   ```

3. Create and activate a virtual environment:
   ```
   uv venv
   source .venv/bin/activate  # On Unix or MacOS
   .venv\Scripts\activate  # On Windows
   ```

4. Install dependencies:
   ```
   uv pip install -r requirements.txt
   ```

## Usage

1. Run the script:
   ```
   python scraper.py
   ```

2. Follow the prompts to enter the product name and number of pages to scrape.

3. The script will save the results in a CSV file named after the search term.

## Features

- User-friendly command-line interface
- Efficient HTML parsing with BeautifulSoup
- CSV output for easy data analysis

## Detailed Explanation

For a comprehensive breakdown of the code and web scraping concepts, see [Scrape_with_Python/info.md](Scrape_with_Python/info.md).

## Ethical Considerations

- Respect Amazon's robots.txt and terms of service.
- Use responsibly to avoid IP blocking.
- Consider implementing delays between requests for ethical scraping.

---

# Amazon Product Scraper (Rust)

This high-performance Rust program concurrently scrapes product information from Amazon India using asynchronous programming.

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager, typically installed with Rust)

## Installation

1. Install Rust:
   - Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
   - Follow the instructions for your operating system

2. Clone and navigate to the repository:
   ```
   git clone https://github.com/CLoaKY233/WebScraper.git
   cd Scrape_with_rust
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Usage

1. Run the program:
   ```
   cargo run --release
   ```

2. Follow the prompts to:
   - Enter the product name
   - Specify the number of pages to scrape
   - Choose whether to print output to console

3. The program will save results in a CSV file named after the search term.

## Features

- Concurrent scraping with Tokio for optimal performance
- User-friendly command-line interface
- CSV output for seamless data analysis
- Built-in rate limiting to respect server resources

## Detailed Explanation

For an in-depth analysis of the Rust implementation and its performance benefits, see [Scrape_with_Rust/info.md](Scrape_with_Rust/info.md).

## Ethical Considerations

- Adhere to Amazon's robots.txt and terms of service.
- Use responsibly to prevent IP blocking.
- The program includes built-in delays to avoid overwhelming the server.

## Performance Comparison

The Rust version typically outperforms the Python version, especially for larger scraping tasks, due to its concurrent design and compiled nature. For specific benchmarks, refer to the detailed explanation document.
