/*
 * gh-notify-daemon - A simple github notification daemon
 * Copyright (C) 2026 2kybe3 <kybe@kybe.xyz>
 */

use reqwest::header;
use serde::Deserialize;
use tokio::task;

use crate::{CONTACT, USER_AGENT, response::Notification};

#[derive(Deserialize)]
struct Response {
    html_url: String,
}

pub async fn log_notification(notification: &Notification, github_token: &str) {
    let handle = match notify_rust::Notification::new()
        .summary(notification.title())
        .body(&notification.body())
        .action("open", "open")
        .finalize()
        .show_async()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to send notification: {e}\n{CONTACT}");
            return;
        }
    };

    let token = github_token.to_owned();
    let url = notification.url().to_owned();
    task::spawn_blocking(move || {
        handle.wait_for_action(|action| {
            if action == "open" {
                tokio::spawn(async move {
                    let client = reqwest::Client::new();
                    let resp = match client
                        .get(&url)
                        .header(header::USER_AGENT, USER_AGENT)
                        .bearer_auth(&token)
                        .send()
                        .await
                    {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("failed to open url: {e}");
                            return;
                        }
                    };

                    let body = match resp.text().await {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("failed to read body: {e}");
                            return;
                        }
                    };

                    let json: Response = match serde_json::from_str(&body) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("failed to parse JSON: {e}");
                            return;
                        }
                    };

                    if let Err(e) = open::that(json.html_url) {
                        eprintln!("failed to open browser: {e}");
                    }
                });
            }
        })
    });
}
