use reqwest;

fn get_html_content(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;
    Ok(html)
}

fn main() {
    let url = "https://example.com";
    match get_html_content(url) {
        Ok(html) => println!("HTML içeriği:\n{}", html),
        Err(err) => println!("Bir hata oluştu: {}", err),
    }
}
