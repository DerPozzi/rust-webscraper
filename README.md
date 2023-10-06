# Rust Webscraper - Assignment by ChatGPT

## Idea

The Idea was to let ChatGP generate a codingassignment, for a $3^{rd}$ semester in Computer Science.
The Assignment is supposed to be in Rust, but the Student has some knowledge of C++ and Python aswell.

### Rust Coding Assignment: Implementing a Concurrent Web Scraper

**Objective:**
The goal of this assignment is to familiarize you with concurrent programming in Rust. You will be implementing a concurrent web scraper that fetches data from multiple URLs concurrently.

**Prerequisites:**

- Proficiency in C++ and Python.
- Basic understanding of concurrent programming concepts.

**Task Description:**
You are required to write a Rust program that concurrently fetches data from a list of URLs and extracts specific information from the HTML content.

**Specifications:**

1. **Input:**
   - Create a file `urls.txt` containing a list of URLs (one URL per line). The URLs should be related to news articles or blog posts.

2. **Output:**
   - The program should output the title and publication date of each article.

3. **Concurrent Fetching:**
   - Implement concurrent fetching of HTML content from the provided URLs. Each URL should be fetched in a separate thread.

4. **HTML Parsing:**
   - Use a library like `select` or `scraper` to parse the HTML content and extract the title and publication date.

5. **Error Handling:**
   - Implement proper error handling. If a URL fails to fetch or parse, the program should continue processing other URLs.

6. **Concurrency Safety:**
   - Ensure that your program handles concurrent access to data structures safely. You can use Rust's `Arc`, `Mutex`, or `RwLock` for this purpose.

7. **Clean Code:**
   - Write clean, well-documented, and idiomatic Rust code. Pay attention to variable names, code structure, and comments.

**Additional Guidelines:**

- You are encouraged to use Rust's asynchronous programming features (async/await) for fetching URLs concurrently. You can use libraries like `reqwest` for making HTTP requests.

- You are allowed to use external libraries for HTML parsing.

- Make sure to provide clear instructions on how to run the program and include any necessary dependencies.

- Include a brief explanation of your code's design and how you ensured concurrency safety.

**Submission:**

- Submit the following files:
  - `main.rs`: The Rust source code file.
  - `urls.txt`: The file containing the list of URLs used for testing.
  - Any additional files or instructions required to run the program.

**Evaluation Criteria:**

- Correctness of the program (Does it extract the title and publication date correctly from the provided URLs?).
- Proper concurrent implementation.
- Error handling and graceful degradation.
- Code quality, readability, and idiomatic Rust usage.
- Comments and documentation.

**Note:** Please ensure that you have the necessary permissions to scrape the content from the provided URLs, and follow any applicable terms of service or policies.
