use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::places::{
    create_item_handler, delete_item_handler, edit_item_handler, get_item_handler,
    items_list_handler,
};

use crate::AppState;

pub fn get_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_item_handler))
        .route("/", get(items_list_handler))
        .route(
            "/{id}",
            get(get_item_handler)
                .patch(edit_item_handler)
                .delete(delete_item_handler),
        )
        .with_state(app_state)
}
