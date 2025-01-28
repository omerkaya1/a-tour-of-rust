#![allow(dead_code)]
use futures::TryStreamExt;
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow)]
struct Message {
    id: u64,
    message: String,
}

async fn update_message(id: u64, msg: &str, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    sqlx::query("UPDATE messages SET message = ? WHERE id = ?;")
        .bind(msg)
        .bind(id as i64)
        .execute(pool)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // run migrations
    // sqlx::migrate!().run(&pool).await?;

    let messages = sqlx::query("select * from messages;")
        .map(|row: sqlx::sqlite::SqliteRow| {
            let id: u64 = row.get(0);
            let msg: String = row.get(1);
            (id, msg)
        })
        .fetch_all(&pool)
        .await?;

    for (id, message) in messages {
        println!("{id} - {message}");
    }

    // data mapping to a struct fields
    let messages = sqlx::query_as::<_, Message>("select * from messages;")
        .fetch_all(&pool)
        .await?;

    println!("{messages:?}");

    update_message(3, "some message", &pool).await?;

    println!("row iteration");

    // streaming
    let mut msg_stream = sqlx::query_as::<_, Message>("select * from messages;")
        .fetch(&pool);

    while let Some(msg) = msg_stream.try_next().await? {
        println!("{msg:?}");
    }

    Ok(())
}
