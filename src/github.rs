/*
 * gh-notify-daemon - A simple github notification daemon
 * Copyright (C) 2026 2kybe3 <kybe@kybe.xyz>
 */

use chrono::{DateTime, Utc};
use reqwest::{StatusCode, header};

use crate::{USER_AGENT, error_notification, response::Notifications};

const NOTIFICATION_ENDPOINT: &str = "https://api.github.com/notifications";
const ACCEPT: &str = "application/vnd.github+json";

const IF_MODIFIED_SINCE_HEADER: &str = "If-Modified-Since";
const API_VERSION_HEADER: &str = "X-GitHub-Api-Version";
const API_VERSION: &str = "2026-03-10";

const LAST_MODIFIED_HEADER: &str = "Last-Modified";
const POOL_INTERVAL_HEADER: &str = "X-Poll-Interval";

const DEFAULT_POOL_INTERVAL: u64 = 60;

#[derive(Debug)]
pub struct GetNotificationResponse {
    pool_interval: u64,
    last_modified: Option<DateTime<Utc>>,
    notifications: Option<Notifications>,
}

impl GetNotificationResponse {
    pub fn new() -> Self {
        Self {
            pool_interval: DEFAULT_POOL_INTERVAL,
            last_modified: None,
            notifications: None,
        }
    }
    pub fn error() -> Self {
        Self::new()
    }

    pub fn not_modified() -> Self {
        Self::new()
    }

    pub fn partial(pool_interval: u64, last_modified: Option<DateTime<Utc>>) -> Self {
        Self {
            pool_interval,
            last_modified,
            notifications: None,
        }
    }

    pub fn full(
        pool_interval: u64,
        last_modified: Option<DateTime<Utc>>,
        notifications: Notifications,
    ) -> Self {
        Self {
            pool_interval,
            last_modified,
            notifications: Some(notifications),
        }
    }

    pub fn pool_interval(&self) -> u64 {
        self.pool_interval
    }

    pub fn last_modified(&self) -> Option<&DateTime<Utc>> {
        self.last_modified.as_ref()
    }

    pub fn notifications(&self) -> Option<&Notifications> {
        self.notifications.as_ref()
    }
}

pub async fn get_notification(
    client: &mut reqwest::Client,
    token: &str,
    newest_notification: Option<&DateTime<Utc>>,
) -> GetNotificationResponse {
    let mut request = client
        .get(NOTIFICATION_ENDPOINT)
        .header(API_VERSION_HEADER, API_VERSION)
        .header(header::USER_AGENT, USER_AGENT)
        .header(header::ACCEPT, ACCEPT)
        .bearer_auth(token);

    if let Some(last_modified) = newest_notification {
        request = request.header(
            IF_MODIFIED_SINCE_HEADER,
            last_modified.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        )
    }

    let res = match request.send().await {
        Ok(v) => v,
        Err(e) => {
            error_notification(format!("error fetching notifications: {e}")).await;
            return GetNotificationResponse::error();
        }
    };

    if let Err(e) = res.error_for_status_ref() {
        error_notification(format!("error fetching notifications: {e}")).await;
        return GetNotificationResponse::error();
    };

    if res.status() == StatusCode::NOT_MODIFIED {
        return GetNotificationResponse::not_modified();
    }

    let pool_interval = match res
        .headers()
        .get(POOL_INTERVAL_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
    {
        Some(v) => v,
        None => {
            error_notification(format!("{POOL_INTERVAL_HEADER} header missing or invalid")).await;
            DEFAULT_POOL_INTERVAL
        }
    };

    let last_modified = res
        .headers()
        .get(LAST_MODIFIED_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| DateTime::parse_from_rfc2822(s).ok())
        .map(|t| t.with_timezone(&Utc));

    let body = match res.text().await {
        Ok(v) => v,
        Err(e) => {
            error_notification(format!("error reading notifications body: {e}")).await;
            return GetNotificationResponse::partial(pool_interval, last_modified);
        }
    };

    let notifications: Notifications = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(e) => {
            error_notification(format!(
                "error parsing notifications body: {e}. Body: {body}"
            ))
            .await;
            return GetNotificationResponse::partial(pool_interval, last_modified);
        }
    };

    GetNotificationResponse::full(pool_interval, last_modified, notifications)
}
