use time::OffsetDateTime;

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn new(
        username: String, 
        email: String, 
        password: String
    ) -> Self {
        Self { 
            id: 0, 
            username, 
            email, 
            password, 
            created_at: OffsetDateTime::now_utc(), 
            updated_at: OffsetDateTime::now_utc() 
        }
    }
}