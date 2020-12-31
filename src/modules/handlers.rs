use crate::modules::interfaces;
use crate::modules::config;

// Handle the giver argument
pub fn handle(args: Vec<String>) {
    match args[0].as_str() {
        "stop" => stop(args),
        "next" => next(false),
        "show" => show(),
        "random" => next(true),
        "config" => config::print(),
        _ => start(args)
    }
}

fn stop(args: Vec<String>) {
    let stoparg = &args[1];
    if str::eq(stoparg, "all") {
        interfaces::set::disable_all();
    } else {
        interfaces::set::disable(stoparg);
    }
}

fn next(random: bool) {
    let next_interface: String;
    if random {
        next_interface = interfaces::get::random();
    } else {
        next_interface = interfaces::get::next();
    }
    start(vec![next_interface.clone()]);
}

fn start(args: Vec<String>) {
    interfaces::set::enable(&args[0]);
}

fn show() {
    interfaces::list::active();
    interfaces::list::available();
}