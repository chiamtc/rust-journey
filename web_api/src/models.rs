use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

// RustcEncodable, RustcDecodable to serialize, encode and decode the data immediately.
#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Post {
    title: String,
    body: String,
    author: String,
    datetime: DateTime<Utc>,
    uuid: Uuid,
}

impl Post {
    pub fn new(title: &str, body: &str, author: &str, datetime: DateTime<Utc>, uuid: Uuid) -> Post {
        Post { title: title.to_string(), body: body.to_string(), author: author.to_string(), datetime, uuid }
    }

    pub fn uuid(&self) -> &Uuid{
        &self.uuid
    }
}