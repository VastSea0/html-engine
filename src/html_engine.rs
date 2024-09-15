use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read};


pub fn parser() -> Vec<String> {
    let mut h_vec = vec![];

    // Dosyayı aç ve tüm içeriği oku
    let mut file = OpenOptions::new()
        .read(true)
        .open("helo.html")
        .expect("Unable to open file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");


    let tags = vec!["h1", "h2", "h3", "h4", "h5", "p"];

    let mut inside_tag = false;
    let mut current_tag = String::new();
    let mut tag_content = String::new();

    for c in content.chars() {
        if c == '<' {
            inside_tag = true;


            if !current_tag.is_empty() && tags.contains(&current_tag.as_str()) {
                h_vec.push(tag_content.trim().to_string());
                tag_content.clear();
            }

            current_tag.clear();
        } else if c == '>' {
            inside_tag = false;
        } else if inside_tag {
            current_tag.push(c);
        } else {
            if !current_tag.is_empty() && tags.contains(&current_tag.as_str()) {
                tag_content.push(c);
            }
        }
    }


    if !current_tag.is_empty() && tags.contains(&current_tag.as_str()) {
        h_vec.push(tag_content.trim().to_string());
    }

    h_vec
}



fn engine() {

}