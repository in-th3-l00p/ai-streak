use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn new(
        id: i32,
        username: String, 
        email: String, 
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime
    ) -> Self {
        Self { 
            id, 
            username, 
            email, 
            created_at, 
            updated_at 
        }
    }
}