use std::process;
use rand::Rng;

use crate::modules::helpers::find::index_of;
use crate::modules::interfaces;

pub fn handle(args: Vec<String>) {
    // Get first argument
    let first_arg = &args[0];
    // Run
    match first_arg.as_str() {
        "stop" => stop(args),
        "next" => next(false),
        "show" => show(args),
        "random" => next(true),
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

fn next(_random: bool) {
    let next_interface: &String;
    let available_interface_names = interfaces::get::available();
    let available_interfaces = available_interface_names.len();
    let active_interface_names = interfaces::get::active();
    let active_interfaces = active_interface_names.len();
    // No available interfaces
    if available_interface_names.len() == 0 {
        println!("No available interfaces.");
        process::exit(1);
    }

    println!("Available interfaces:");
    for i in &available_interface_names {
        println!("{}", i);
    }
    println!("Active interfaces:");
    for i in &active_interface_names {
        println!("{}", i);
    }
    // Enable first available
    if active_interfaces == 0 {
        next_interface = &available_interface_names[0];
    } else {
        // Get current interface index
        let mut current_index = index_of(
            &available_interface_names,
            &active_interface_names[0]
        ) as i32;
        // If at last index
        if current_index >= available_interfaces as i32 - 1 {
            current_index = -1;
        }
        // Set next
        let next_index = current_index + 1;
        next_interface = &available_interface_names[(next_index as usize)];
    
    }
    // Drop all before setting next
    stop(vec!["".to_string(), "all".to_string()]);
    // Start next
    start(vec![next_interface.clone()]);
}

fn random() {
    let available_interface_names = interfaces::get::available();
    let available_interfaces = available_interface_names.len();
    let active_interface_names = interfaces::get::active();
    // No available interfaces
    if available_interface_names.len() == 0 {
        println!("No available interfaces.");
        process::exit(1);
    }
    // Randomize and perevent getting same interface
    let mut random_index: usize = 0;
    let mut rng = rand::thread_rng();
    if active_interface_names.len() > 0 {
        // Get current interface index
        let current_index = index_of(
            &available_interface_names,
            &active_interface_names[0]
        ) as usize;
        // Get random
        let mut done = false;
        while !done {
            random_index = rng.gen_range(0 .. available_interfaces);
            if random_index != current_index {
                done = true;
            }
        }
    } else {
        random_index = rng.gen_range(0 .. available_interfaces);
    }

    let random_interface = &available_interface_names[random_index];
    // Start interface
    // Drop all before setting next
    stop(vec!["".to_string(), "all".to_string()]);
    // Start next
    start(vec![random_interface.clone()]);
}

fn start(args: Vec<String>) {
    // Drop all before setting next
    stop(vec!["".to_string(), "all".to_string()]);
    // Start
    let startarg = &args[0];
    interfaces::set::enable(startarg);
}

fn show(args: Vec<String>) {
    // Show active if no args
    if args.len() <= 1 {
        interfaces::list::active();
        process::exit(1);
    }
    // List available if args is "all"
    let showarg = &args[1];
    if str::eq(showarg, "all") {
        interfaces::list::available();
    } else {
        println!("Invalid argument!")
    }
    
}
