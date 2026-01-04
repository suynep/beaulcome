#[derive(Debug)]
pub enum AvailFlags {
    Init,      // initialize the message
    Display,   // preview the startup message
    Permanent, // make the display permanent (add to $HOME/.zshrc or $HOME/.bashrc, will support more shells later)
}

#[derive(Debug)]
pub struct Flag(AvailFlags);

pub fn read_all_flags() -> Vec<Flag> {
    let flags: Vec<String> = std::env::args().skip(1).collect();
    let mut map: Vec<Flag> = Vec::new();

    for i in 0..flags.len() {
        match &(flags[i][..]) {
            "init" => map.push(Flag(AvailFlags::Init)),
            "display" => map.push(Flag(AvailFlags::Display)),
            "permanent" => map.push(Flag(AvailFlags::Permanent)),
            _ => {}
        }
    }
    map
}
