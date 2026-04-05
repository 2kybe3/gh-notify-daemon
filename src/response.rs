/*
 * gh-notify-daemon - A simple github notification daemon
 * Copyright (C) 2026 2kybe3 <kybe@kybe.xyz>
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Notifications(Vec<Notification>);
impl Notifications {
    pub fn notifications(&self) -> &Vec<Notification> {
        &self.0
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Notification {
    subject: Subject,
    repository: Repository,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Repository {
    full_name: String,
    description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Subject {
    title: String,
    #[serde(rename = "type")]
    type_: String,
}

impl Notification {
    pub fn title(&self) -> &str {
        &self.subject.title
    }

    pub fn body(&self) -> String {
        format!(
            "Repo: {}\nDescription: {}\nType: {}",
            self.repository.full_name, self.repository.description, self.subject.type_
        )
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
