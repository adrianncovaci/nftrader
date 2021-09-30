use std::sync::Arc;

use models::{feed::Feed, user::User};
use tokio::sync::RwLock;

pub mod models;
pub mod chat;

pub type Users = Arc<RwLock<Vec<User>>>;
pub type GlobalFeed = Arc<RwLock<Feed>>;
