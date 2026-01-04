use std::{env::home_dir, fs, io};

// pub const _MSG_PATH: &str = "/home/s01is/.beaulcomemsg";

pub fn _save_msg(_msg: &str) -> io::Result<()> {
    let mut home = home_dir().unwrap();
    home.push(".beaulcomemsg");
    fs::write(home, _msg)
}

pub fn get_msg() -> String {
    let mut home = home_dir().unwrap();
    home.push(".beaulcomemsg");
    if !fs::exists(&home).ok().unwrap() {
        let _ = _save_msg("<GENERIC_INITIALIZATION_MSG>");
    }

    let opt = fs::read_to_string(home).ok();

    if let Some(value) = opt {
        value.to_lowercase()
    } else {
        String::from("")
    }
}
