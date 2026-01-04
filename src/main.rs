mod args;
mod ascii;
mod setup;

fn main() {
    let msg = setup::get_msg();

    if msg != "" {
        println!("Received: {msg}");
    } else {
        println!(
            "Couldn't read the message file at {0}",
            "$HOME/.beaulcomemsg"
        );
    }

    let map = args::read_all_flags();

    if !map.is_empty() {
        for m in map.clone() {
            match m {
                args::Flag(args::AvailFlags::Init(s)) => {
                    let _ = setup::_save_msg(&s);
                }
                args::Flag(args::AvailFlags::Display) => {
                    let data = ascii::join_graphemes(ascii::print_message(&setup::get_msg())); // test purposes, as of now
                    println!("{data}");
                }
                args::Flag(args::AvailFlags::Update(s)) => {
                    let _ = setup::_save_msg(&s);
                }
                args::Flag(args::AvailFlags::Permanent) => {
                    println!("Permanence Flag triggered");
                }
            }
        }
    }

    println!("{:?}", map);

    let data = ascii::join_graphemes(ascii::print_message(&setup::get_msg())); // test purposes, as of now

    println!("{data}");
}
