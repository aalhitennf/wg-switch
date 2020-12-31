pub mod set {

    use std::process;

    use crate::modules::checkers::interface_exists;
    use crate::modules::helpers;
    use crate::modules::interfaces;
    use crate::modules::services;

    use crate::modules::config::CONFIG;

    pub fn disable_all() {
        let active_interfaces = interfaces::get::active();
        for interface in active_interfaces {
            disable(&interface);
        }
    }

    pub fn disable(interface: &str) {
        if !interface_exists(String::from(interface)) {
            println!("No interface '{}'", interface);
            process::exit(1);
        }
        let args: &[&str] = &["down", interface];
        let command = ("wg-quick", args);
        // Disable interface
        helpers::run::with_shell(command);
        // Disable and stop service
        if CONFIG.systemd {
            println!("Disabling systemd service");
            services::systemd::set(interface, "disable");
        }
        
    }

    pub fn enable(interface: &str) {
        if !interface_exists(String::from(interface)) {
            println!("No interface '{}'", interface);
            process::exit(1);
        }
        let args: &[&str] = &["up", interface];
        let command = ("wg-quick", args);
        println!("Enabling {}", interface);
        // Disable others before enabling
        disable_all();
        // Enable interface
        helpers::run::with_shell(command);
        // Enable and start service
        if CONFIG.systemd {
            println!("Enabling systemd service");
            services::systemd::set(interface, "enable");
        }

    }

}

pub mod get {

    use std::process;
    use rand::Rng;

    use crate::modules::helpers;

    pub fn active() -> Vec<String> {
        let command: (&str, &[&str]) = ("wg", &["show"]);
        let output = helpers::run::with_shell(command);
        helpers::trim::interfaces(output)
    }

    pub fn available() -> Vec<String> {
        let mut available: Vec<String> = Vec::new();
        let files = std::fs::read_dir("/etc/wireguard").unwrap();
        for file in files {
            let as_string = String::from(
                file.unwrap().path().into_os_string().into_string().unwrap()
            );
            let parts: Vec<&str> = as_string.split("/").collect();
            let conf_name = String::from(parts[parts.len() - 1])
                .replace(".conf", "");
            available.push(conf_name);
        }
        available.sort();
        available
    }

    // Return next interface name from available in conf directory
    pub fn next() -> String {
        // Get available and active
        let next_interface: &str;
        let available_names = available();
        let available_amount = available_names.len() as i32;
        let active_names = active();
        let active_amount = active_names.len() as i32;
        // Exit if no available
        if available_amount == 0 {
            println!("No available interfaces.");
            process::exit(1);
        }
        // Get first available if none active
        if active_amount == 0 {
            next_interface = &available_names[0];
        } else { // Otherwise get next
            // Get current interface index
            let mut current_index = helpers::find::index_of_string(
                &available_names, &active_names[0]
            );
            // If at last index
            if current_index >= available_amount - 1 {
                current_index = -1;
            }
            // Set next
            let next_index = current_index + 1;
            next_interface = &available_names[(next_index as usize)];
        }
        String::from(next_interface)
    }

    // Returns random interface name that is not currently active
    pub fn random() -> String {
        // Get available and active
        let available_names = available();
        let available_amount = available_names.len() as i32;
        let active_names = active();
        let active_amount = active_names.len() as i32;
        // No available interfaces
        if available_names.len() == 0 {
            println!("No available interfaces.");
            process::exit(1);
        }
        // Randomize and perevent getting same interface
        let mut random_index: i32 = 0;
        let mut rng = rand::thread_rng();
        if active_amount > 0 {
            // Get current interface index
            let current_index = helpers::find::index_of_string(
                &available_names, &active_names[0]
            );
            // Get random
            let mut done = false;
            while !done {
                random_index = rng.gen_range(0 .. available_amount);
                if random_index != current_index {
                    done = true;
                }
            }
        } else {
            random_index = rng.gen_range(0 .. available_amount);
        }
        String::from(&available_names[(random_index as usize)])
    }

}

pub mod list {

    use crate::modules::interfaces;

    pub fn active() {
        let active_interfaces = interfaces::get::active();
        println!("Active interfaces:");
        for interface in active_interfaces {
            println!("{}", interface);
        }
    }

    pub fn available() {
        let available_interfaces = interfaces::get::available();
        println!("Available interfaces:");
        for interface in available_interfaces {
            println!("{}", interface);
        }
    }

}
