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

pub fn print_message(msg: &str) -> Vec<String> {
    let mut all_loaded: Vec<String> = Vec::new();
    for m in msg.chars() {
        if get_char_path(m).is_some() {
            let content = load_letter(get_char_path(m).unwrap());
            // println!("{}", content);
            all_loaded.push(content);
        } else if get_char_path(m).is_none() {
            if m.is_ascii_alphabetic() {
                let content = load_letter(m.to_string());
                // println!("{}", content);
                all_loaded.push(content);
            }
        }
    }

    all_loaded
}

pub fn join_graphemes(graphemes: Vec<String>) {
    /*
     * TODO: line-wise join for the 12-lines of each letter in letter/
     */
    let mut overall = String::new();
    let spaces_between = String::from(" ").repeat(2);
    let cloned_g = graphemes.clone();
    let first_str_split: Vec<&str> = cloned_g.first().unwrap().split("\n").collect();
    let invariant_len: usize = first_str_split.len();

    for i in 0..invariant_len {
        // we move forward with one crucial assumption: Every letter's file contains the same
        // number of lines in total

        for g in graphemes.clone() {
            let all_split: Vec<&str> = g.split("\n").collect();
            let ith_split = all_split[i];
            overall.push_str(ith_split);
            overall.push_str(&spaces_between);
        }
        overall.push('\n');
    }

    println!("{overall}");
}
