#[derive(Debug, Clone)]
pub enum AvailFlags {
    Init(String),   // initialize the message
    Display,        // preview the startup message
    Permanent, // make the display permanent (add to $HOME/.zshrc or $HOME/.bashrc, will support more shells later)
    Update(String), // -> updation mechanism :'(
}

#[derive(Debug, Clone)]
pub struct Flag(pub AvailFlags);

pub fn read_all_flags() -> Vec<Flag> {
    let flags: Vec<String> = std::env::args().skip(1).collect();
    let mut map: Vec<Flag> = Vec::new();

    for i in 0..flags.len() {
        match &(flags[i][..]) {
            "init" => map.push(Flag(AvailFlags::Init("Default Message".to_string()))),
            "display" => map.push(Flag(AvailFlags::Display)),
            "permanent" => map.push(Flag(AvailFlags::Permanent)),
            "update" => {
                if flags.get(i + 1).is_some() {
                    map.push(Flag(AvailFlags::Update(flags[i + 1].clone())));
                }
            }
            _ => {}
        }
    }

    map
}

// the above flags are implemented in the main function
