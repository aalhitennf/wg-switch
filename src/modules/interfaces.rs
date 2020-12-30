pub mod set {

    use std::process;
    use crate::modules::checkers::interface_exists;
    use crate::modules::helpers;
    use crate::modules::interfaces;
    use crate::modules::services;

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
        let args: [&str; 2] = ["down", interface];
        let command = ("wg-quick", args);
        // Disable interface
        helpers::run::with_shell(command);
        // Disable and stop service
        services::systemd::set(interface, "disable");
    }

    pub fn enable(interface: &str) {
        if !interface_exists(String::from(interface)) {
            println!("No interface '{}'", interface);
            process::exit(1);
        }
        let args: [&str; 2] = ["up", interface];
        let command = ("wg-quick", args);
        println!("Enabling {}", interface);
        // Enable interface
        helpers::run::with_shell(command);
        // Enable and start service
        services::systemd::set(interface, "enable");
    }

}

pub mod get {

    use crate::modules::helpers;
    use crate::modules::interfaces;

    pub fn active() -> Vec<String> {
        let command: (&str, [&str; 1]) = ("wg", ["show"]);
        let output = helpers::run::shell_with_output(command);
        return helpers::trim::interfaces(output);

    }

    pub fn available() -> Vec<String> {
        let mut available: Vec<String> = Vec::new();
        let files = std::fs::read_dir("/etc/wireguard").unwrap();
        for file in files {
            let as_string = String::from(file.unwrap().path().into_os_string().into_string().unwrap());
            let parts: Vec<&str> = as_string.split("/").collect();
            let mut conf_name = String::from(parts[parts.len() - 1]);
            conf_name = conf_name.replace(".conf", "");
            available.push(conf_name);
        }
        available.sort();
        return available;
    }

    pub fn next() {

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
