use ssh2::Session;

pub fn disconnect_ssh(session: Option<Session>, message: &mut String) {
    if let Some(sess) = session {
        if let Err(e) = sess.disconnect(None, "Bye bye", None) {
            *message = format!("Failed to disconnect: {}", e);
        } else {
            *message = "Successfully disconnected".to_string();
        }
    } else {
        *message = "No active session to disconnect".to_string();
    }
}
