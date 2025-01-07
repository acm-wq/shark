use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Entry, Button, Box, Label, Orientation, CheckButton};
use serde_json::json;
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::rc::Rc;

const TIMER_INTERVAL_SECS: u32 = 3600;

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

    app.connect_activate(move |app| {
        let window = Rc::new(ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Shark")
            .build());

        let vbox = Box::new(Orientation::Vertical, 5);

        let entry_words = Rc::new(Entry::new());
        entry_words.set_placeholder_text(Some("Enter words separated by spaces"));

        let entry_translations = Rc::new(Entry::new());
        entry_translations.set_placeholder_text(Some("Enter translations separated by spaces"));

        let button = Button::with_label("Save to JSON");
        let button_show_words = Button::with_label("Show words");

        let entry_words_clone = Rc::clone(&entry_words);
        let entry_translations_clone = Rc::clone(&entry_translations);
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
            }
        });

        let app_clone = app.clone();
        button_show_words.connect_clicked(move |_| {
            let words_window = ApplicationWindow::builder()
                .application(&app_clone)
                .default_width(400)
                .default_height(600)
                .title("Words")
                .build();

            let words_vbox = Box::new(Orientation::Vertical, 5);
            words_window.set_child(Some(&words_vbox));

            let words = load_words_from_file();
            for (word, translation) in words {
                let hbox = Box::new(Orientation::Horizontal, 5);
                
                let check_button = CheckButton::new();
                let translation_label = Label::new(Some(&translation));

                let word_label = Rc::new(Label::new(Some(&word)));
                word_label.set_visible(false);

                let word_label_clone = Rc::clone(&word_label);

                check_button.connect_toggled(move |btn| {
                    word_label_clone.set_visible(btn.is_active());
                });

                hbox.append(&check_button);
                hbox.append(&translation_label);
                hbox.append(&*word_label);
                words_vbox.append(&hbox);
            }
            words_window.present();
        });

        vbox.append(&*entry_words);
        vbox.append(&*entry_translations);
        vbox.append(&button);
        vbox.append(&button_show_words);

        window.set_child(Some(&vbox));
        window.present();

        let show_words_in_new_window = {
            let app_clone = app.clone();
            move || {
                let words = load_words_from_file();
                let words_window = ApplicationWindow::builder()
                    .application(&app_clone)
                    .default_width(400)
                    .default_height(600)
                    .title("Words")
                    .build();

                let words_vbox = Box::new(Orientation::Vertical, 5);
                words_window.set_child(Some(&words_vbox));

                for (word, translation) in words {
                    let hbox = Box::new(Orientation::Horizontal, 5);
                    
                    let check_button = CheckButton::new();
                    let translation_label = Label::new(Some(&translation));

                    let word_label = Rc::new(Label::new(Some(&word)));
                    word_label.set_visible(false);

                    let word_label_clone = Rc::clone(&word_label);

                    check_button.connect_toggled(move |btn| {
                        word_label_clone.set_visible(btn.is_active());
                    });

                    hbox.append(&check_button);
                    hbox.append(&translation_label);
                    hbox.append(&*word_label);
                    words_vbox.append(&hbox);
                }
                words_window.present();
                glib::ControlFlow::Continue
            }
        };

        glib::timeout_add_seconds_local(TIMER_INTERVAL_SECS, show_words_in_new_window);
    });

    app.run()
}