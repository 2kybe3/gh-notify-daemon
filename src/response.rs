/*
 * gh-notify-daemon - A simple github notification daemon
 * Copyright (C) 2026 2kybe3 <kybe@kybe.xyz>
 */

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Notifications(Vec<Notification>);
impl Notifications {
    pub fn notifications(&self) -> &Vec<Notification> {
        &self.0
    }
}

#[derive(Deserialize, Debug)]
pub struct Notification {
    subject: Subject,
    repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    full_name: String,
    description: String,
}

#[derive(Deserialize, Debug)]
pub struct Subject {
    title: String,
    url: String,
    #[serde(rename = "type")]
    type_: String,
}

impl Notification {
    pub fn title(&self) -> &str {
        &self.subject.title
    }

    pub fn url(&self) -> &str {
        &self.subject.url
    }

    pub fn body(&self) -> String {
        format!(
            "Repo: {}\nDescription: {}\nType: {}",
            self.repository.full_name, self.repository.description, self.subject.type_
        )
    }
}
