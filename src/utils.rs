// Copyright 2016-2017 Chang Lan
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use libc;
use log::info;
use std::process::Command;

/// Checks if the current process is running with root privileges.
///
/// # Returns
///
/// * `bool` - Returns `true` if the current process is running as root, otherwise `false`.
pub fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

/// Enables IPv4 forwarding on the system.
///
/// This function enables IP forwarding for IPv4 packets by modifying the appropriate
/// kernel parameter via the `sysctl` command. It is designed to work with Linux and macOS.
///
/// # Returns
///
/// * `Result<(), String>` - Returns `Ok(())` if IPv4 forwarding is successfully enabled.
///   If an error occurs, returns `Err(String)` containing the error message.
///
/// # Panics
///
/// This function will panic if the target OS is neither Linux nor macOS.
pub fn enable_ipv4_forwarding() -> Result<(), String> {
    let sysctl_arg = if cfg!(target_os = "linux") {
        "net.ipv4.ip_forward=1"
    } else if cfg!(target_os = "macos") {
        "net.inet.ip.forwarding=1"
    } else {
        unimplemented!()
    };
    info!("Enabling IPv4 Forwarding.");
    let status = Command::new("sysctl")
        .arg("-w")
        .arg(sysctl_arg)
        .status()
        .unwrap();
    if status.success() {
        Ok(())
    } else {
        Err(format!("sysctl: {}", status))
    }
}

pub enum RouteType {
    Net,
    Host,
}

pub struct DefaultGateway {
    origin: String,
    remote: String,
    default: bool,
}

impl DefaultGateway {
    /// Creates a new `DefaultGateway` instance and modifies the system routing table.
    ///
    /// This function creates a new `DefaultGateway` instance, saves the original default gateway,
    /// adds a route to the remote host through the original default gateway, and optionally
    /// replaces the default gateway with the provided `gateway`.
    ///
    /// # Arguments
    ///
    /// * `gateway` - A string slice representing the new default gateway's IP address.
    /// * `remote` - A string slice representing the remote host's IP address.
    /// * `default` - A boolean indicating whether to replace the current default gateway.
    ///
    /// # Returns
    ///
    /// * `DefaultGateway` - A new instance of the `DefaultGateway` struct.
    pub fn create(gateway: &str, remote: &str, default: bool) -> DefaultGateway {
        let origin = get_default_gateway().unwrap();
        info!("Original default gateway: {}.", origin);
        add_route(RouteType::Host, remote, &origin).unwrap();
        if default {
            delete_default_gateway().unwrap();
            set_default_gateway(gateway).unwrap();
        }
        DefaultGateway {
            origin: origin,
            remote: String::from(remote),
            default: default,
        }
    }
}

impl Drop for DefaultGateway {
    /// Restores the original default gateway and removes the added route when the `DefaultGateway`
    /// instance is dropped.
    ///
    /// This function is called automatically when the `DefaultGateway` instance goes out of scope.
    /// It restores the original default gateway if it was replaced, and removes the added route
    /// to the remote host.
    fn drop(&mut self) {
        if self.default {
            delete_default_gateway().unwrap();
            set_default_gateway(&self.origin).unwrap();
        }
        delete_route(RouteType::Host, &self.remote).unwrap();
    }
}

/// Deletes a route from the system routing table.
///
/// This function deletes a route of the specified type from the system routing table using the
/// `route` command. It is designed to work with Linux and macOS.
///
/// # Arguments
///
/// * `route_type` - An enum value of `RouteType`, specifying whether the route is a network
///   or a host route.
/// * `route` - A string slice representing the route's IP address or network.
///
/// # Returns
///
/// * `Result<(), String>` - Returns `Ok(())` if the route is successfully deleted. If an error
///   occurs, returns `Err(String)` containing the error message.
///
/// # Panics
///
/// This function will panic if the target OS is neither Linux nor macOS.
pub fn delete_route(route_type: RouteType, route: &str) -> Result<(), String> {
    let mode = match route_type {
        RouteType::Net => "-net",
        RouteType::Host => "-host",
    };
    info!("Deleting route: {} {}.", mode, route);
    let status = if cfg!(target_os = "linux") {
        Command::new("route")
            .arg("-n")
            .arg("del")
            .arg(mode)
            .arg(route)
            .status()
            .unwrap()
    } else if cfg!(target_os = "macos") {
        Command::new("route")
            .arg("-n")
            .arg("delete")
            .arg(mode)
            .arg(route)
            .status()
            .unwrap()
    } else {
        unimplemented!()
    };
    if status.success() {
        Ok(())
    } else {
        Err(format!("route: {}", status))
    }
}

/// Adds a route to the system routing table.
///
/// This function adds a route of the specified type to the system routing table using the
/// `route` command. It is designed to work with Linux and macOS.
///
/// # Arguments
///
/// * `route_type` - An enum value of `RouteType`, specifying whether the route is a network
///   or a host route.
/// * `route` - A string slice representing the route's IP address or network.
/// * `gateway` - A string slice representing the gateway's IP address.
///
/// # Returns
///
/// * `Result<(), String>` - Returns `Ok(())` if the route is successfully added. If an error
///   occurs, returns `Err(String)` containing the error message.
///
/// # Panics
///
/// This function will panic if the target OS is neither Linux nor macOS.
pub fn add_route(route_type: RouteType, route: &str, gateway: &str) -> Result<(), String> {
    let mode = match route_type {
        RouteType::Net => "-net",
        RouteType::Host => "-host",
    };
    info!("Adding route: {} {} gateway {}.", mode, route, gateway);
    let status = if cfg!(target_os = "linux") {
        Command::new("route")
            .arg("-n")
            .arg("add")
            .arg(mode)
            .arg(route)
            .arg("gw")
            .arg(gateway)
            .status()
            .unwrap()
    } else if cfg!(target_os = "macos") {
        Command::new("route")
            .arg("-n")
            .arg("add")
            .arg(mode)
            .arg(route)
            .arg(gateway)
            .status()
            .unwrap()
    } else {
        unimplemented!()
    };
    if status.success() {
        Ok(())
    } else {
        Err(format!("route: {}", status))
    }
}


/// Sets the system's default gateway.
///
/// # Arguments
///
/// * `gateway` - A string slice representing the IP address of the new default gateway.
///
/// # Returns
///
/// * `Result<(), String>` - Returns `Ok(())` if the default gateway is successfully set. If an error
///   occurs, returns `Err(String)` containing the error message.
pub fn set_default_gateway(gateway: &str) -> Result<(), String> {
    add_route(RouteType::Net, "default", gateway)
}

/// Deletes the system's default gateway.
///
/// # Returns
///
/// * `Result<(), String>` - Returns `Ok(())` if the default gateway is successfully deleted. If an error
///   occurs, returns `Err(String)` containing the error message.
pub fn delete_default_gateway() -> Result<(), String> {
    delete_route(RouteType::Net, "default")
}

/// Retrieves the system's current default gateway.
///
/// This function is designed to work with Linux and macOS.
///
/// # Returns
///
/// * `Result<String, String>` - Returns `Ok(String)` containing the IP address of the current default
///   gateway if successful. If an error occurs, returns `Err(String)` containing the error message.
///
/// # Panics
///
/// This function will panic if the target OS is neither Linux nor macOS.
pub fn get_default_gateway() -> Result<String, String> {
    let cmd = if cfg!(target_os = "linux") {
        "ip -4 route list 0/0 | awk '{print $3}'"
    } else if cfg!(target_os = "macos") {
        "route -n get default | grep gateway | awk '{print $2}'"
    } else {
        unimplemented!()
    };
    let output = Command::new("bash").arg("-c").arg(cmd).output().unwrap();
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)
            .unwrap()
            .trim_right()
            .to_string())
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}

/// Retrieves the public IP address of the system.
///
/// # Returns
///
/// * `Result<String, String>` - Returns `Ok(String)` containing the public IP address if successful.
///   If an error occurs, returns `Err(String)` containing the error message.
pub fn get_public_ip() -> Result<String, String> {
    let output = Command::new("curl")
        .arg("ipecho.net/plain")
        .output()
        .unwrap();
    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}

/// Retrieves the gateway for a specific route.
///
/// # Arguments
///
/// * `route` - A string slice representing the route's IP address or network.
///
/// # Returns
///
/// * `Result<String, String>` - Returns `Ok(String)` containing the gateway IP address if successful.
///   If an error occurs, returns `Err(String)` containing the error message.
fn get_route_gateway(route: &str) -> Result<String, String> {
    let cmd = format!("ip -4 route list {}", route);
    let output = Command::new("bash").arg("-c").arg(cmd).output().unwrap();
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)
            .unwrap()
            .trim_right()
            .to_string())
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}

/// Sets the system's DNS resolver.
///
/// # Arguments
///
/// * `dns` - A string slice representing the IP address of the DNS server to use.
///
/// # Returns
///
/// * `Result<String, String>` - Returns `Ok(String)` if the DNS resolver is successfully set.
///   If an error occurs, returns `Err(String)` containing the error message.
///
/// # Safety
///
/// This function overwrites the `/etc/resolv.conf` file and requires root privileges.
pub fn set_dns(dns: &str) -> Result<String, String> {
    let cmd = format!("echo nameserver {} > /etc/resolv.conf", dns);
    let output = Command::new("bash").arg("-c").arg(cmd).output().unwrap();
    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}

/// Flushes the system's DNS cache.
///
/// # Returns
///
/// * `Result<String, String>` - Returns `Ok(String)` if the DNS cache is successfully flushed.
///   If an error occurs, returns `Err(String)` containing the error message.
///
/// # Safety
///
/// This function uses the `sudo` command to grant the necessary privileges to flush the DNS cache.
/// Ensure that the user running this function has the appropriate permissions
/// configured in the `/etc/sudoers` file.
pub fn flush_dns() -> Result<String, String> {
    let cmd = "sudo systemd-resolve --flush-caches";
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap();

    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::*;

    #[test]
    fn enable_ipv4_forwarding_test() {
        enable_ipv4_forwarding().unwrap();
    }
    #[test]
    #[cfg(target_os = "linux")]
    fn get_default_gateway_test() {
        let a = get_default_gateway().unwrap();
        assert!(get_route_gateway("0/0").unwrap().contains(&*a))
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn route_test() {
        assert!(is_root());
        let gw = get_default_gateway().unwrap();
        add_route(RouteType::Host, "1.1.1.1", &gw).unwrap();
        assert!(get_route_gateway("1.1.1.1").unwrap().contains(&*gw));
        delete_route(RouteType::Host, "1.1.1.1").unwrap();
        assert!(!get_route_gateway("1.1.1.1").unwrap().contains(&*gw));
    }
    #[test]
    fn set_dns_test() {
        assert!(is_root());
        set_dns("8.8.8.8").unwrap();
    }
}
