//! Twitch API to get a list of viewers for single streams.

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    chatters: Chatters,
}

#[derive(Deserialize)]
struct Chatters {
    viewers: Vec<String>,
}

/// Get a list of currently active viewers for a Twitch username.
pub fn get_viewers(username: &str) -> Result<Vec<String>> {
    let url = format!("https://tmi.twitch.tv/group/user/{}/chatters", username);
    Ok(attohttpc::get(url)
        .send()?
        .json::<Response>()?
        .chatters
        .viewers)
}
