use std::{collections::HashMap, fs};

fn load_letter(letter: String) -> String {
    let letter_type: &str = "bigmoney-sw";
    let letters_path: &str = &format!("letters/{letter_type}");
    let letter_content = fs::read_to_string(format!("{letters_path}/{letter}"));

    letter_content.ok().unwrap()
}

fn get_char_path(c: char) -> Option<String> {
    let mut extra_chars = HashMap::<char, String>::new();
    extra_chars.insert(',', "comma".to_string());
    extra_chars.insert('!', "bang".to_string());
    extra_chars.insert('.', "period".to_string());
    extra_chars.insert('?', "question".to_string());

    if extra_chars.contains_key(&c) {
        Some(extra_chars.get(&c).unwrap().to_string())
    } else {
        None
    }
}

pub fn print_message(msg: &str) {
    for m in msg.chars() {
        if get_char_path(m).is_some() {
            let content = load_letter(get_char_path(m).unwrap());
            println!("{}", content);
        } else if get_char_path(m).is_none() {
            if m.is_ascii_alphabetic() {
                let content = load_letter(m.to_string());
                println!("{}", content);
            }
        }
    }
}

pub fn join_graphemes(graphemes: Vec<String>) {}
