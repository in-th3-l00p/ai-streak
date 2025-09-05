use time::OffsetDateTime;

pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub is_public: bool,        
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Note {
    pub fn new(
        user_id: i32, 
        title: String, 
        content: String, 
        is_public: bool
    ) -> Self {
        Self { 
            id: 0, 
            user_id, 
            title, 
            content, 
            is_public, 
            created_at: OffsetDateTime::now_utc(), 
            updated_at: OffsetDateTime::now_utc() 
        }
    }
}