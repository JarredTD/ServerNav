use ssh2::Session;
use std::net::{TcpStream, ToSocketAddrs};

pub fn connect_to_ssh(
    address: &str,
    username: &str,
    password: &str,
    message: &mut String,
) -> Option<Session> {
    let mut addrs = match address.to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(e) => {
            *message = format!("Failed to resolve address {}: {}", address, e);
            return None;
        }
    };

    if let Some(socket_addr) = addrs.next() {
        match TcpStream::connect(socket_addr) {
            Ok(tcp) => {
                let mut sess = Session::new().unwrap();
                sess.set_tcp_stream(tcp);
                if let Err(e) = sess.handshake() {
                    *message = format!("Failed to handshake: {}", e);
                    return None;
                }

                if sess.userauth_password(username, password).is_ok() {
                    if sess.authenticated() {
                        *message = format!("Successfully authenticated to {}", address);
                        return Some(sess);
                    } else {
                        *message = format!("Authentication failed to {}", address);
                    }
                } else {
                    *message =
                        "Failed to authenticate with the given username and password.".to_string();
                }
            }
            Err(e) => {
                *message = format!("Failed to connect to '{}': {}", address, e);
            }
        }
    } else {
        *message = format!("Failed to resolve address: {}", address);
    }
    None
}
