use anyhow::anyhow;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use common::domain::user::User;
use sqlx::PgPool;
use std::sync::Arc;

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
            record.created_at.assume_utc(),
            record.updated_at.assume_utc(),
        ))
    }

    fn get_argon2(self: &Self) -> anyhow::Result<Argon2> {
        Ok(Argon2::new_with_secret(
            self.secret.as_ref(),
            Algorithm::Argon2id,
            Version::V0x13,
            Params::default(),
        )
            .map_err(|err| anyhow!(err.to_string()))?)
    }

    async fn read_login(
        self: &Self,
        username: String,
        password: String
    ) -> anyhow::Result<User> {
        let argon2 = self.get_argon2()?;
        let record = sqlx::query!(
            r#"
                select id, username, email, password, created_at, updated_at from users
                where email = $1 or username = $1
            "#,
            username
        )
            .fetch_one(self.pool.as_ref())
            .await?;
        let hash = PasswordHash::new(&record.password)
            .map_err(|err| anyhow!(err.to_string()))?;
        argon2
            .verify_password(password.as_bytes(), &hash)
            .map_err(|err| anyhow!(err.to_string()))?;
        Ok(User::new(
            record.id,
            record.username,
            record.email,
            record.created_at.assume_utc(),
            record.updated_at.assume_utc(),
        ))
    }

    pub async fn login(
        self: &Self,
        username: String,
        password: String
    ) -> anyhow::Result<User> {
        self.read_login(username, password)
            .await
            .map_err(|_| anyhow!("wrong email or password"))
    }

    fn hash_password(self: &Self, password: String) -> anyhow::Result<String> {
        let argon2 = self.get_argon2()?;
        Ok(argon2
            .hash_password(password.as_ref(), self.salt.as_salt())
            .expect("failed to hash password")
            .to_string())
    }

    pub async fn create(
        self: &Self,
        username: String,
        email: String,
        password: String,
    ) -> anyhow::Result<User> {
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
            self.hash_password(password)?
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        self.read(record.id).await
    }

    pub async fn update(
        self: &Self,
        id: i32,
        username: String,
        email: String,
        password: String,
    ) -> anyhow::Result<User> {
        sqlx::query!(
            r#"
                update users set
                    username = $1,
                    email = $2,
                    password = $3,
                    updated_at = now()
                where id = $4
            "#,
            username,
            email,
            self.hash_password(password)?,
            id
        )
        .execute(self.pool.as_ref())
        .await?;

        self.read(id).await
    }

    pub async fn delete(self: &Self, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                delete from users
                where id = $1
            "#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
