use serde_json::json;

use crate::WEBHOOK;

#[allow(unused_must_use)]
pub async fn send_webhook(
    content: &str,
    author: &str,
    title: &str,
    description: &str,
    hex_color: u32
) -> () {
    let client = reqwest::Client::new();

    let embed = json!({
        "author": {
            "name": author
        },
        "title": title,
        "description": description,
        "color": hex_color
    });

    let payload = json!({
        "content": content,
        "embeds": [embed]
    });

    client
        .post(WEBHOOK)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;
}