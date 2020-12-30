use nix::unistd::getuid;

use crate::modules::interfaces;

pub fn interface_exists(interface: String) -> bool {
    let available_interfaces = interfaces::get::available();
    return available_interfaces.contains(&interface);
}

pub fn is_root() -> bool {
    return getuid().as_raw() == 0;
}
