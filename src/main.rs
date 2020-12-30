use std::env;
use std::process;

mod modules;

fn main() {
    // Return if not root
    if modules::checkers::is_root() == false {
        println!("You need root privileges.");
        process::exit(1);
    }
    // Return if no args
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 0 {
        println!("No args!");
        process::exit(1);
    }
    // Handle
    modules::handlers::handle(args);
}
