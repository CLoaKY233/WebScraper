# Rust Web Scraping Code Explanation

This Rust script performs web scraping on Amazon India, similar to the Python version, but with significant performance improvements due to concurrent processing.

## Key Components:

1. **Libraries**: The script uses `tokio` for async runtime, `reqwest` for HTTP requests, `scraper` for HTML parsing, and `csv` for file output.

2. **Structs**: `Product` for holding product data, `Selectors` for storing CSS selectors.

3. **fetch_page function**: Asynchronously fetches and parses a single page of results.

4. **main function**: Orchestrates the entire scraping process, including user input and concurrent execution.

## How it works:

1. User inputs search term, number of pages, and output preference
2. The script creates tasks for each page to be scraped concurrently
3. Each task sends an async GET request and parses the HTML
4. Product information is extracted using CSS selectors
5. Results are collected and optionally printed
6. Data is written to a CSV file

## Why it's faster than Python:

1. **Concurrency**: Rust uses async/await and tokio for efficient concurrent processing.
2. **Memory efficiency**: Rust's ownership system and lack of garbage collection reduce overhead.
3. **Compiled language**: Rust compiles to native code, eliminating interpreter overhead.
4. **Zero-cost abstractions**: Rust's high-level features don't impact runtime performance.

## Things needed to learn:

1. Rust programming language basics
2. Asynchronous programming in Rust
3. Tokio runtime for async I/O
4. Error handling in Rust
5. Structs and traits
6. Working with external crates (libraries)

## 10 Prompts for Understanding the Application:

1. Explain the role of the tokio runtime in this Rust web scraping application.
2. How does Rust's async/await syntax contribute to the script's performance?
3. Describe the process of creating and joining multiple tasks for concurrent scraping.
4. How does Rust's type system help in structuring the scraped data?
5. Explain the use of Arc (Atomic Reference Counting) in sharing data between threads.
6. How does the script handle errors during the scraping process?
7. Describe the process of writing the scraped data to a CSV file in Rust.
8. How does Rust's ownership system contribute to the memory efficiency of this script?
9. Explain the role of the reqwest library in making HTTP requests.
10. How could this script be further optimized for even better performance?
```
