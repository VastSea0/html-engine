### HTML Engine (Rust-based Experimental Browser)

![HTML Engine in example.com ](https://github.com/user-attachments/assets/3fd7fdf2-d7f0-49e1-aa05-3f10676d420d)

This project is an experimental HTML engine built with Rust, combining web scraping, file system manipulation, and a graphical user interface (GUI) using GTK. It demonstrates how to fetch and parse HTML content from a webpage, extract specific elements, and display them in a GTK-based window.

#### Features
- **Web Scraping with Reqwest:** Fetches HTML content from a URL using the `reqwest` library.
- **HTML Parsing:** Extracts specific HTML tags (`<h1>`, `<h2>`, `<h3>`, `<h4>`, `<h5>`, and `<p>`) and stores their content.
- **File Handling with OpenOptions:** Saves the fetched HTML to a file and allows further reading and writing operations.
- **GTK Integration for GUI:** Displays the parsed content in a user-friendly GTK window, with labels for each HTML tag.
  
#### How it Works
1. **Fetching HTML Content:**
   The HTML content of a given URL is retrieved using `reqwest`. This is done through the `get_html_content` function, which handles HTTP requests and returns the HTML as a `String`.

2. **Saving and Reading HTML:**
   The `fs` function saves the HTML content to a local file (`helo.html`). This allows the engine to reuse the data for future parsing and display.

3. **Parsing HTML Tags:**
   The `parser` function scans the HTML file, extracting the contents of specific tags like `h1`, `h2`, and `p`. These elements are then stored in a `Vec<String>` and displayed in the GUI.

4. **Displaying Content with GTK:**
   The GTK-based GUI displays the URL, HTML content, and parsed elements in a vertical layout. Each element (e.g., `h1` or `p`) is displayed as a separate label within the window.

#### Installation
To build and run this project, you'll need:
- Rust toolchain (install via [rustup](https://rustup.rs/))
- GTK 3 development libraries
- `reqwest` crate for web scraping
- `gtk` crate for the GUI

Install dependencies:
```bash
cargo install reqwest
cargo install gtk
```

Clone the repository and run the application:
```bash
git clone https://github.com/vastsea0/html-engine
cd html-engine
cargo run
```

#### Usage
1. Modify the URL in the `main()` function to the desired webpage.
2. Run the application. The HTML content will be fetched, saved to `helo.html`, and parsed.
3. A GTK window will open, showing the URL and parsed HTML tags (`h1`, `h2`, etc.).

#### Example Output
When pointing to a URL like `https://html5doctor.com/`, the engine will extract and display all `h1`, `h2`, and paragraph tags in a GTK window.

#### Future Improvements
- **CSS Parsing and Rendering:** Add support for basic CSS styles to enhance rendering capabilities.
- **Mini JavaScript Handling:** Evaluate JavaScript within the HTML for more dynamic content.
- **Improved Parsing:** Expand the parser to handle more complex HTML structures.

#### License
This project is licensed under the MIT License. See the `LICENSE` file for more details.
