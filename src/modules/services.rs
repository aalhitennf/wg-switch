pub mod systemd {

    use crate::modules::helpers::run::with_shell;

    pub fn set(interface: &str, action: &str) {
        let service_name = ["wg-quick@", interface, ".service"].join("");
        let args: [&str; 2] = [action, &service_name];
        let command = ("systemctl", args);
        println!("{} {}", action, command.1[1]);
        with_shell(command);
    }

}