use std::env;
use std::process;

mod modules;

fn main() {

    // Return if not root
    if modules::checkers::is_root() == false {
        println!("Requires root privileges.");
        process::exit(1);
    }

    // Return if no args
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 0 {
        println!("Available arguments:");
        println!("%interface - enable interface with given name");
        println!("show - list active and available interfaces");
        println!("stop {{%interface, all}}- stop interface with given name or all");
        println!("next - activate next available interface (alphabetically sorted)");
        println!("random - activate random interface");
        process::exit(1);
    }
    // Handle
    modules::handlers::handle(args);
}
