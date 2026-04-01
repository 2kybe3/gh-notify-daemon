/*
 * gh-notify-daemon - A simple github notification daemon
 * Copyright (C) 2026 2kybe3 <kybe@kybe.xyz>
 */

use chrono::{DateTime, Utc};

use crate::{CONTACT, error_notification, response::Notification};

pub async fn log_notification(
    notification: &Notification,
    newest_notification: Option<DateTime<Utc>>,
) {
    if let Some(newest_notification) = newest_notification
        && newest_notification <= notification.updated_at()
    {
        return;
    }
    match serde_json::to_string(notification) {
        Ok(v) => println!("{v}"),
        Err(e) => error_notification(format!("{e}")).await,
    };

    if let Err(e) = notify_rust::Notification::new()
        .summary(notification.title())
        .body(&notification.body())
        .action("open", "open")
        .finalize()
        .show_async()
        .await
    {
        eprintln!("failed to send notification: {e}\n{CONTACT}");
    };
}
