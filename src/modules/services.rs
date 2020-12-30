pub mod systemd {

    use crate::modules::helpers::run::with_shell;

    pub fn set(interface: &str, action: &str) {
        let service_name = ["wg-quick@", interface, ".service"].join("");
        let args: &[&str] = &[action, &service_name];
        let command = ("systemctl", args);
        with_shell(command);
    }

}
