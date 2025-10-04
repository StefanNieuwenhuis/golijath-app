use crate::models::archives::Archive;
use crate::schemas::archives::{CreateArchive, UpdateArchive};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use std::sync::Arc;

use crate::AppState;

const TABLE: &str = "archives";

/**
 * List Items Handler
 * This handler fetches a list of all items from postgres
 */
pub async fn items_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = format!("SELECT * FROM {} ORDER BY name", TABLE);
    let query_result = sqlx::query_as::<_, Archive>(&query)
        .fetch_all(&data.db)
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
    let query_result = sqlx::query_as::<_, Archive>(&query)
        .bind(id)
        .fetch_one(&data.db)
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
    Json(body): Json<CreateArchive>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let query_result = sqlx::query_as::<_, Archive>(&query)
        .bind(body.name.to_string())
        .fetch_one(&data.db)
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
    Json(body): Json<UpdateArchive>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
    let query_result = sqlx::query_as::<_, Archive>(&query)
        .bind(id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("🔥 Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let item = query_result.unwrap();

    let query = format!("UPDATE {} SET name = $1 WHERE id = $2 RETURNING *", TABLE);
    let query_result = sqlx::query_as::<_, Archive>(&query)
        .bind(body.name.to_owned().unwrap_or(item.name))
        .bind(id)
        .fetch_one(&data.db)
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
        .execute(&data.db)
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
