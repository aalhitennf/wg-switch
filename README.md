**wg-switch**

Simple wrapper for quick switching wireguard interfaces on linux systems. It reads wireguard config filenames from `/etc/wireguard` and uses `wg-quick` to enable/disable and `systemctl` to enable/disable services. We want this to happen without problems so enabling one disables every other active interface.

**dependencies**
`wireguard-tools` - Should be found from almost every distributions repository.

**usage**
`wg-switch` - Show this.
`wg-switch %interface%` - Enable interface with given name.
`wg-switch show` - List active and available interfaces.
`wg-switch stop {%interface%, all}` - Stop interface with given name or all.
`wg-switch next` - Activate next available interface (alphabetically sorted).
`wg-switch random` - Activate random interface.

`%interface%` is name of config file i.e. enabling `/etc/wireguard/wg0.conf` use `wg-switch wg0`.

**building**
1. Install rust.
2. Build with `cargo build --release`.

**install**
Run script `install.sh` or just copy `target/release/wg-switch` to `/usr/bin` manually.
