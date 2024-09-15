use std::fs::OpenOptions;
use std::io::Write;

pub fn fs(command: &str, content: String){
    let mut html_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // Eğer dosya yoksa oluştur
        .open("helo.html")
        .expect("Unable to open file");

    match command {
        "write" => {
            html_file.write_all(content.as_bytes()).expect("Failed to write to file");
        }
        _ => {}
    }

}