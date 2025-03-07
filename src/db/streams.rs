use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use sqlx::{types::Uuid, PgPool};

#[derive(sqlx::FromRow, Debug, Deserialize, Clone)]
pub struct Stream {
    pub id: Uuid,
    pub user_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub video_id: Option<String>,
    pub games: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Serialize for Stream {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Stream", 7)?; // 7 fields (excluding user_id)

        state.serialize_field("id", &self.id)?;
        state.serialize_field("start_time", &self.start_time)?;
        state.serialize_field("end_time", &self.end_time)?;
        state.serialize_field("games", &self.games)?;
        state.serialize_field("video_id", &self.video_id)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("updated_at", &self.updated_at)?;

        state.end()
    }
}

impl Stream {
    /// Creates a new stream instance (not persisted)
    pub fn new(user_id: Uuid, start_time: DateTime<Utc>) -> Self {
        Stream {
            id: Uuid::new_v4(),
            user_id,
            start_time,
            end_time: None,
            games: None,
            video_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Creates a new stream in the database
    pub async fn create(
        user_id: Uuid,
        start_time: DateTime<Utc>,
        pool: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        let stream = Stream::new(user_id, start_time);

        sqlx::query_as::<_, Stream>(
            "INSERT INTO streams (
                id, user_id, start_time, end_time, games, video_id
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *",
        )
        .bind(stream.id)
        .bind(stream.user_id)
        .bind(stream.start_time)
        .bind(stream.end_time)
        .bind(&stream.games)
        .bind(&stream.video_id)
        .fetch_one(pool)
        .await
    }

    /// Finds a stream by ID
    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM streams WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    /// Finds all streams
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM streams ORDER BY start_time DESC")
            .fetch_all(pool)
            .await
    }

    /// Finds all streams for a specific user
    pub async fn find_by_user_id(user_id: Uuid, pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM streams WHERE user_id = $1 ORDER BY start_time DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Finds active streams for a specific user
    pub async fn find_active_by_user_id(
        user_id: Uuid,
        pool: &PgPool,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM streams
            WHERE user_id = $1 AND end_time IS NULL
            ORDER BY start_time DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Gets the most recent active stream for a user
    pub async fn find_most_recent_active_by_user_id(
        user_id: Uuid,
        pool: &PgPool,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM streams
            WHERE user_id = $1 AND end_time IS NULL
            ORDER BY start_time DESC
            LIMIT 1",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
    }

    /// Finds all active streams (no end time)
    pub async fn find_active(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM streams WHERE end_time IS NULL ORDER BY start_time DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Updates the stream end time
    pub async fn end_stream(
        &mut self,
        end_time: DateTime<Utc>,
        pool: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query(
            "UPDATE streams
            SET end_time = $1,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $2",
        )
        .bind(end_time)
        .bind(self.id)
        .execute(pool)
        .await?;

        self.end_time = Some(end_time);
        self.updated_at = Utc::now();
        Ok(self.clone())
    }

    /// Updates the stream's video URL
    pub async fn set_video(&mut self, video_id: String, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE streams
            SET vide_id = $1,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $2",
        )
        .bind(video_id.clone())
        .bind(self.id)
        .execute(pool)
        .await?;

        self.video_id = Some(video_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Deletes a stream
    pub async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM streams WHERE id = $1")
            .bind(self.id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
