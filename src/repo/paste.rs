use std::error::Error;

use chrono::NaiveDateTime;

use sqlx::PgPool;
use uuid::Uuid;

pub async fn add_paste<Ip: Into<sqlx::types::ipnetwork::IpNetwork>>(
    pool: &PgPool,
    content: String,
    ip: Ip,
) -> Result<Uuid, Box<dyn Error>> {
    let rec = sqlx::query!(
        r#"
INSERT INTO pastes (id, content, ip)
VALUES ( $1, $2, $3 )
RETURNING id
        "#,
        Uuid::new_v4(),
        content,
        ip.into()
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

#[derive(sqlx::FromRow)]
pub struct Paste {
    pub id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub user_id: Option<i32>,
    pub ip: sqlx::types::ipnetwork::IpNetwork,
}

pub async fn get_paste(pool: &PgPool, id: Uuid) -> Result<Paste, Box<dyn Error>> {
    let paste = sqlx::query_as!(Paste, "SELECT * FROM pastes WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(paste)
}
