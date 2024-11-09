use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Video {
    pub id: String,
    pub user_id: Uuid,
    pub title: String,
    pub raw_video_path: String,
    pub processed_video_path: Option<String>,
    pub processing_status: ProcessingStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "processing_status", rename_all = "lowercase")]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl Video {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        title: String,
        raw_video_path: String,
    ) -> Result<Self, sqlx::Error> {
        let video_id = nanoid!(10);
        sqlx::query_as::<_, Video>(
            r#"
            INSERT INTO videos (id, user_id, title, raw_video_path, processing_status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, user_id, title, raw_video_path, processed_video_path,
                      processing_status, created_at, updated_at
            "#,
        )
        .bind(video_id)
        .bind(user_id)
        .bind(title)
        .bind(raw_video_path)
        .fetch_one(pool)
        .await
    }
}
