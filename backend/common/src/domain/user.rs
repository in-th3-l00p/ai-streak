use time::PrimitiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl User {
    pub fn new(
        id: i32,
        username: String, 
        email: String, 
        created_at: PrimitiveDateTime,
        updated_at: PrimitiveDateTime
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