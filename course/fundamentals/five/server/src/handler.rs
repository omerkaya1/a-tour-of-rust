use axum::{Extension, Json, extract::Path};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct DataPoint {
    id: i32,
    collector_id: String,
    received: i64,
    total_memory: i64,
    used_memory: i64,
    average_cpu: f32,
}

const SHOW_ALL_QUERY: &str = "SELECT * FROM timeseries;";

pub async fn show_all(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>(SHOW_ALL_QUERY)
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}

#[derive(Debug, FromRow, Serialize)]
pub struct CollectorData {
    id: i32,
    collector_id: String,
    last_seen: i64,
}

const SHOW_COLLECTORS_QUERY: &str = "
SELECT
	DISTINCT(id) AS id,
	collector_id,
	(SELECT MAX(received) FROM timeseries WHERE collector_id = ts.collector_id) AS last_seen
FROM timeseries ts;
";

pub async fn show_collectors(
    Extension(pool): Extension<sqlx::SqlitePool>,
) -> Json<Vec<CollectorData>> {
    let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(
        sqlx::query_as::<_, CollectorData>(SHOW_COLLECTORS_QUERY)
            .fetch_all(&pool)
            .await
            .unwrap(),
    )
}

const SHOW_COLLECTOR_DATA_QUERY: &str = "SELECT * FROM timeseries WHERE collector_id = ? ORDER BY received;";

pub async fn collector_data(Extension(pool): Extension<sqlx::SqlitePool>, id: Path<String>) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>(SHOW_COLLECTOR_DATA_QUERY)
        .bind(id.as_str())
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}