use ssh2::Session;

pub fn disconnect_ssh(session: Option<Session>) -> Result<String, String> {
    if let Some(sess) = session {
        sess.disconnect(None, "Bye bye", None)
            .map_err(|e| format!("Failed to disconnect: {}", e))?;
    } else {
        return Ok("No active session to disconnect".to_string());
    }
    Ok("Successfully disconnected".to_string())
}
