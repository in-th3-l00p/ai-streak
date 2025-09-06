use anyhow::anyhow;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use common::domain::user::User;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    pool: Arc<PgPool>,
    secret: Vec<u8>,
    salt: SaltString,
}

impl UserService {
    pub fn new(pool: Arc<PgPool>, secret: Vec<u8>) -> Self {
        Self { 
            pool,
            secret,
            salt: SaltString::generate(&mut OsRng),
        }
    }

    pub async fn read(self: &Self, id: i32) -> anyhow::Result<User> {
        let record = sqlx::query!(
            r#"
                select id, username, email, created_at, updated_at from users
                where id = $1
            "#,
            id
        )
            .fetch_one(self.pool.as_ref())
            .await?;

        Ok(User::new(
            record.id,
            record.username,
            record.email,
            record.created_at.assume_utc()  ,
            record.updated_at.assume_utc(),
        ))
    }

    pub async fn create(
        self: &Self,
        username: String,
        email: String,
        password: String,
    ) -> anyhow::Result<User> {
        let argon2 = Argon2::new_with_secret(
            self.secret.as_ref(),
            Algorithm::Argon2id,
            Version::V0x13,
            Params::default()
        )
            .map_err(|err| anyhow!(err.to_string()))?;
        let hashed = argon2
            .hash_password(password.as_ref(), self.salt.as_salt())
            .expect("failed to hash password")
            .to_string();

        let record = sqlx::query!(
            r#"
                insert into users (
                    username,
                    email,
                    password,
                    created_at,
                    updated_at
                )
                    values ($1, $2, $3, now(), now())
                    returning id
            "#,
            username,
            email,
            hashed
        )
            .fetch_one(self.pool.as_ref())
            .await?;

        self.read(record.id).await
    }
}