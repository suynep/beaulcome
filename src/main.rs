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
    println!("{:?}", map);

    // ascii::print_message("hello, world");
}
