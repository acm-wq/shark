use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Entry, Button, Box, Label};
use serde_json::json;
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::rc::Rc;

fn load_words_from_file() -> Vec<(String, String)> {
    let mut file = match File::open("src/storage/words.json") {
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
        .filter_map(|v| {
            if let Some(obj) = v.as_object() {
                if let (Some(word), Some(translation)) = (obj.get("word"), obj.get("translation")) {
                    if let (Some(word_str), Some(translation_str)) = (word.as_str(), translation.as_str()) {
                        return Some((word_str.to_string(), translation_str.to_string()));
                    }
                }
            }
            None
        })
        .collect()
}

fn save_to_archive(words: &[(String, String)]) {
    let archive_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("src/storage/archive.json")
        .expect("Failed to open archive file");

    let json_data = json!({ "words": words.iter().map(|(w, t)| json!({ "word": w, "translation": t })).collect::<Vec<_>>() });

    serde_json::to_writer(archive_file, &json_data).expect("Failed to write to archive file");
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
            .title("Shark")
            .build();

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        
        let entry_words = Rc::new(Entry::new());
        entry_words.set_placeholder_text(Some("Enter words separated by spaces"));

        let entry_translations = Rc::new(Entry::new());
        entry_translations.set_placeholder_text(Some("Enter translations separated by spaces"));

        let button = Button::with_label("Save to JSON");
        
        let label = Label::new(Some("List of words:"));
        
        let words = load_words_from_file();
        if !words.is_empty() {
            let words_display: Vec<String> = words.iter()
                .map(|(word, translation)| format!("{}: {}", word, translation))
                .collect();
            label.set_text(&format!("List of words: {}", words_display.join(", ")));
        }

        let entry_words_clone = Rc::clone(&entry_words);
        let entry_translations_clone = Rc::clone(&entry_translations);
        let label_clone = label.clone();
        button.connect_clicked(move |_| {
            let words_text = entry_words_clone.text().to_string();
            let translations_text = entry_translations_clone.text().to_string();

            let words: Vec<&str> = words_text.split_whitespace().collect();
            let translations: Vec<&str> = translations_text.split_whitespace().collect();

            if words.len() == translations.len() {
                save_to_archive(&load_words_from_file());

                let word_pairs: Vec<(String, String)> = words.iter()
                    .zip(translations.iter())
                    .map(|(w, t)| (w.to_string(), t.to_string()))
                    .collect();

                let json_data = json!({ "words": word_pairs.iter().map(|(w, t)| json!({ "word": w, "translation": t })).collect::<Vec<_>>() });

                let file = File::create("src/storage/words.json").expect("Failed to create file");
                serde_json::to_writer(file, &json_data).expect("Failed to write to file");

                let words_display: Vec<String> = word_pairs.iter()
                    .map(|(w, t)| format!("{}: {}", w, t))
                    .collect();
                label_clone.set_text(&format!("List of words: {}", words_display.join(", ")));
            } else {
                label_clone.set_text("The number of words and translations must match.");
            }
        });

        vbox.append(&*entry_words);
        vbox.append(&*entry_translations);
        vbox.append(&button);
        vbox.append(&label);
        
        window.set_child(Some(&vbox));

        window.present();
    });

    app.run()
}
