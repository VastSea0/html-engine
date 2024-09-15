mod fs;
mod html_engine;

use reqwest;
use crate::fs::fs;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use crate::html_engine::parser;

fn get_html_content(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;
    Ok(html)
}

fn main() {

    let url = "https://html5doctor.com/";
    match get_html_content(url) {
        Ok(html) => {
            println!("HTML içeriği:\n{}", html);

            let fs_html = html.clone();
            fs("write", fs_html.to_string());

            let vector = parser();//parser(html, url.to_string());
            browser_window(html, url.to_string(), vector);

        },
        Err(err) => println!("Bir hata oluştu: {}", err),
    }


}

fn browser_window (html_content: String, window_title: String, h: Vec<String>) {
    let application = Application::builder()
        .application_id("com.browser.HtmlEngine")
        .build();


    let title = format!("URL: {}", window_title);
    application.connect_activate(move |app| {
        let window_title_clone = title.clone();
        let window = ApplicationWindow::builder()
            .application(app)
            .title(&window_title_clone)
            .default_width(1024)
            .default_height(768)
            .build();

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let url_label = gtk::Label::builder().label(title.as_str()).build();
        //let html_label = gtk::Label::builder().label(html_content.to_string()).build();

        // SIRALAMA
        vbox.add(&url_label);
        for element in &h {
            let h1_label = gtk::Label::builder().label(element.as_str()).build();
            vbox.add(&h1_label);
        }

        //vbox.add(&html_label);



        window.add(&vbox);

        window.show_all();
    });

    application.run();
}
