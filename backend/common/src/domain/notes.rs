use time::PrimitiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub is_public: bool,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Note {
    pub fn new(
        id: i32,
        user_id: i32,
        title: String,
        content: String,
        is_public: bool,
        created_at: PrimitiveDateTime,
        updated_at: PrimitiveDateTime,
    ) -> Self {
        Self {
            id,
            user_id,
            title,
            content,
            is_public,
            created_at,
            updated_at
        }
    }
}