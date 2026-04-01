/*
 * gh-notify-daemon - A simple github notification daemon
 * Copyright (C) 2026 2kybe3 <kybe@kybe.xyz>
 */

use std::{process::exit, time::Duration};

use tokio::fs;

use crate::display::log_notification;

mod display;
mod github;
pub mod response;

const CONTACT: &str = "If the problem persists, please contact me: https://kybe.xyz/ident.txt";
const NO_TOKEN_MSG: &str = "Please set the \"GH_NOTIFY_DAEMON_TOKEN\" or \"GH_NOTIFY_DAEMON_TOKEN_FILE\" env variable\nTo obtain the TOKEN go to https://github.com/settings/tokens and create a classic token with notifications perms";
const USER_AGENT: &str = "gh-notify-daemon / https://git.kybe.xyz/2kybe3/gh-notify-daemon";

#[tokio::main]
async fn main() {
    let github_token = get_token().await;
    run_loop(&github_token).await;
}

async fn get_token() -> String {
    if let Ok(path) = std::env::var("GH_NOTIFY_DAEMON_TOKEN_FILE") {
        let token = fs::read_to_string(path).await;
        match token {
            Ok(t) => return t,
            Err(e) => {
                error_notification(format!("error reading token file: {e}")).await;
            }
        }
    };

    match std::env::var("GH_NOTIFY_DAEMON_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            error_notification_contact(NO_TOKEN_MSG, false).await;
            exit(1);
        }
    }
}

async fn run_loop(github_token: &str) {
    eprintln!(
        "gh-notify-daemon - A simple github notification daemon\nCopyright (C) 2026 2kybe3 <kybe@kybe.xyz>\n\n"
    );

    let mut client = reqwest::Client::new();
    let mut newest_notification = None;

    loop {
        let res =
            github::get_notification(&mut client, github_token, newest_notification.as_ref()).await;

        if let Some(last_modified) = res.last_modified() {
            newest_notification = Some(*last_modified);
        }

        if let Some(notifications) = res.notifications() {
            for notification in notifications.notifications() {
                log_notification(notification, github_token).await;
            }
        }

        tokio::time::sleep(Duration::from_secs(res.pool_interval())).await;
    }
}

async fn error_notification_contact(error: impl AsRef<str>, contact: bool) {
    let msg = format!("{}\n{CONTACT}", error.as_ref());
    eprintln!("{}", &msg);
    if let Err(e) = notify_rust::Notification::new()
        .summary("gh-notify-daemon ERROR")
        .body(&msg)
        .finalize()
        .show_async()
        .await
    {
        eprintln!(
            "failed to send notification: {e}\n{}",
            if contact { CONTACT } else { "" }
        );
    }
}

pub async fn error_notification(error: impl AsRef<str>) {
    error_notification_contact(error, true).await;
}
