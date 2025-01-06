use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Entry, Button, Box, Label};
use serde_json::json;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

fn load_words_from_file() -> Vec<String> {
    let mut file = match File::open("src/words.json") {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return Vec::new();
    }

    let json_data: Value = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(_) => return Vec::new(),
    };

    json_data["words"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect()
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        
        let entry = Rc::new(Entry::new());
        entry.set_placeholder_text(Some("Enter words with a space"));

        let button = Button::with_label("Save in JSON");
        
        let label = Label::new(Some("List of words:"));
        
        let words = load_words_from_file();
        if !words.is_empty() {
            label.set_text(&format!("List of words: {}", words.join(", ")));
        }

        let entry_clone = Rc::clone(&entry);
        let label_clone = label.clone();
        button.connect_clicked(move |_| {
            let text = entry_clone.text().to_string();
            let words: Vec<&str> = text.split_whitespace().collect();

            let json_data = json!({ "words": words });

            let file = File::create("src/words.json").expect("Failed to create file");
            serde_json::to_writer(file, &json_data).expect("Failed to write to file");

            let words_display = words.join(", ");
            label_clone.set_text(&format!("List of words: {}", words_display));
        });

        vbox.append(&*entry);
        vbox.append(&button);
        vbox.append(&label);
        
        window.set_child(Some(&vbox));

        window.present();
    });

    app.run()
}
