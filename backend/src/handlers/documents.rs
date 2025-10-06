use crate::db::AppState;
use crate::models::documents::Document;
use crate::schemas::documents::{CreateDocument, UpdateDocument};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use std::sync::Arc;

const TABLE: &str = "documents";

/**
 * List Items Handler
 * This handler fetches a list of all items from postgres
 */
pub async fn items_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = format!("SELECT * FROM {}", TABLE);
    let query_result = sqlx::query_as::<_, Document>(&query)
        .fetch_all(data.pool())
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "🔥 Something bad happened while fetching all items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let items = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": items.len(),
        "items": items
    });
    Ok(Json(json_response))
}

/**
 * Fetch a single Item
 */
pub async fn get_item_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
    let query_result = sqlx::query_as::<_, Document>(&query)
        .bind(id)
        .fetch_one(data.pool())
        .await;

    match query_result {
        Ok(item) => {
            let item_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "item": item
            })});

            return Ok(Json(item_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Item with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

/**
 * Create Item Handler
 * This handler adds a new item to postgres
 */
pub async fn create_item_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateDocument>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = format!(
        r#"
        INSERT INTO {}
            (date, inventory_number, scan_number, page_number, notes, archive_id, institute_id, place_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
    "#,
        TABLE
    );

    let query_result = sqlx::query_as::<_, Document>(&query)
        .bind(body.date)
        .bind(body.inventory_number)
        .bind(body.scan_number)
        .bind(body.page_number)
        .bind(body.notes)
        .bind(body.archive_id)
        .bind(body.institute_id)
        .bind(body.place_id)
        .fetch_one(data.pool())
        .await;
    match query_result {
        Ok(item) => {
            let item_response = json!({"status": "success","data": json!({
                "item": item
            })});

            return Ok((StatusCode::CREATED, Json(item_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Item with that name already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

/**
 * Edit Item Handler
 * This handler handles edits of existing items
 */
pub async fn edit_item_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateDocument>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
    let query_result = sqlx::query_as::<_, Document>(&query)
        .bind(id)
        .fetch_one(data.pool())
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("🔥 Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let item = query_result.unwrap();

    let query = format!(
        r#"
        INSERT INTO {}
            (date, inventory_number, scan_number, page_number, notes, archive_id, institute_id, place_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
    "#,
        TABLE
    );

    let query_result = sqlx::query_as::<_, Document>(&query)
        .bind(body.date.unwrap_or_else(|| chrono::Utc::now())) // provide default for required date
        .bind(body.inventory_number.unwrap_or(item.inventory_number))
        .bind(body.scan_number.unwrap_or(item.scan_number))
        .bind(body.page_number.unwrap_or(item.page_number))
        .bind(body.notes.unwrap_or(item.notes))
        .bind(body.archive_id.unwrap_or(item.archive_id))
        .bind(body.institute_id.unwrap_or(item.institute_id))
        .bind(body.place_id.unwrap_or(item.place_id))
        .fetch_one(data.pool())
        .await;
    match query_result {
        Ok(item) => {
            let item_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "item": item
            })});

            return Ok(Json(item_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

/**
 * Delete Item Handler
 */
pub async fn delete_item_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    let rows_affected = sqlx::query(&query)
        .bind(id) // $1
        .execute(data.pool())
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
