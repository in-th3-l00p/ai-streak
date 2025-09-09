use anyhow::anyhow;
use sqlx::PgPool;
use std::sync::Arc;
use common::domain::notes::Note;

pub struct NoteService {
    pool: Arc<PgPool>,
}

impl NoteService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn read(&self, id: i32) -> anyhow::Result<Note> {
        let record = sqlx::query!(
            r#"
                select id, user_id, title, content, is_public, created_at, updated_at
                from notes
                where id = $1
            "#,
            id
        )
            .fetch_one(self.pool.as_ref())
            .await?;

        Ok(Note::new(
            record.id,
            record.user_id,
            record.title,
            record.content,
            record.is_public,
            record.created_at,
            record.updated_at,
        ))
    }

    pub async fn create(
        &self,
        user_id: i32,
        title: String,
        content: String,
        is_public: bool,
    ) -> anyhow::Result<Note> {
        let record = sqlx::query!(
            r#"
                insert into notes (
                    user_id,
                    title,
                    content,
                    is_public,
                    created_at,
                    updated_at
                )
                values ($1, $2, $3, $4, now(), now())
                returning id
            "#,
            user_id,
            title,
            content,
            is_public,
        )
            .fetch_one(self.pool.as_ref())
            .await?;

        self.read(record.id).await
    }

    pub async fn update(
        &self,
        id: i32,
        title: String,
        content: String,
        is_public: bool,
    ) -> anyhow::Result<Note> {
        sqlx::query!(
            r#"
                update notes
                set title = $1,
                    content = $2,
                    is_public = $3,
                    updated_at = now()
                where id = $4
            "#,
            title,
            content,
            is_public,
            id
        )
            .execute(self.pool.as_ref())
            .await?;

        self.read(id).await
    }

    pub async fn delete(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                delete from notes
                where id = $1
            "#,
            id
        )
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn list_by_user(&self, user_id: i32) -> anyhow::Result<Vec<Note>> {
        let records = sqlx::query!(
            r#"
                select id, user_id, title, content, is_public, created_at, updated_at
                from notes
                where user_id = $1
                order by created_at desc
            "#,
            user_id
        )
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(records
            .into_iter()
            .map(|record| {
                Note::new(
                    record.id,
                    record.user_id,
                    record.title,
                    record.content,
                    record.is_public,
                    record.created_at,
                    record.updated_at,
                )
            })
            .collect())
    }
}