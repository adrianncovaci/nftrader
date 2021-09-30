use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Message {
    id: Uuid,
    user_id: Uuid,
    content: String,
    posted_date: DateTime<Utc>
}

impl Message {
    pub fn new(user_id: Uuid, content: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            content: content.into(),
            posted_date: Utc::now()
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_user(&self) -> Uuid {
        self.user_id
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn get_posted_date(&self) -> DateTime<Utc> {
        self.posted_date
    }
}
