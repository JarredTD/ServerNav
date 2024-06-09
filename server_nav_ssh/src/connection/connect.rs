use ssh2::Session;
use std::net::{TcpStream, ToSocketAddrs};

pub fn connect_to_ssh(
    address: &str,
    username: &str,
    password: &str,
) -> Result<Option<Session>, String> {
    let mut addrs = address
        .to_socket_addrs()
        .map_err(|e| format!("Failed to resolve address {}, {}", address, e))?;

    if let Some(socket_addr) = addrs.next() {
        match TcpStream::connect(socket_addr) {
            Ok(tcp) => {
                let mut sess = Session::new().unwrap();
                sess.set_tcp_stream(tcp);
                sess.handshake()
                    .map_err(|e| format!("Failed to handshake: {}", e))?;

                if sess.userauth_password(username, password).is_ok() {
                    if sess.authenticated() {
                        return Ok(Some(sess));
                    } else {
                        return Err(format!("Authentication failed to {}", address));
                    }
                } else {
                    return Err(
                        "Failed to authenticate with the given username and password.".to_string(),
                    );
                }
            }
            Err(e) => {
                return Err(format!("Failed to connect to '{}': {}", address, e));
            }
        }
    } else {
        return Err(format!("Failed to resolve address: {}", address));
    }
}
