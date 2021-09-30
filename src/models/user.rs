use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    nickname: String,
    joined: DateTime<Utc>
}

impl User {
    pub fn new(id: Uuid, nickname: &str) -> Self {
        Self {
            id,
            nickname: nickname.into(),
            joined: Utc::now()
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_nickname(&self) -> String {
        self.nickname.clone()
    }

    pub fn get_joined_date(&self) -> DateTime<Utc> {
        self.joined
    }
}
