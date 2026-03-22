use axum::{Json, extract::{State, Path}, http::StatusCode, response::IntoResponse};
use shared::types::{Db, UpdateRequest};
use sqlite::State as SqliteState;

pub async fn update_url(Path(short_code): Path<String>,State(db): State<Db>,Json(update_request): Json<UpdateRequest>,) -> impl IntoResponse {

     // validating URL
    if update_request.new_original_url.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "URL cannot be empty"}))
        ).into_response()
    }

    if !update_request.new_original_url.starts_with("http://") && 
       !update_request.new_original_url.starts_with("https://") {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "URL must start with http:// or https://"}))
        ).into_response()
    }




    let connection = match db.lock() {
        Ok(conn) => conn,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database lock failed"}))
        ).into_response()
    };

    // checking if short_code exists
    let select_query = "SELECT * FROM shorten_url WHERE short_code = ?";
    let mut statement = match connection.prepare(select_query) {
        Ok(stmt) => stmt,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to prepare query: {}", e)}))
        ).into_response()
    };
    statement.bind((1, short_code.as_str())).unwrap();

    if let Ok(SqliteState::Row) = statement.next() {
        // if short_code exists run update
        let update_query = "UPDATE shorten_url SET original_url = ? WHERE short_code = ?";
        let mut update_statement = match connection.prepare(update_query) {
            Ok(stmt) => stmt,
            Err(e) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to prepare update: {}", e)}))
            ).into_response()
        };

        update_statement.bind((1, update_request.new_original_url.as_str())).unwrap();
        update_statement.bind((2, short_code.as_str())).unwrap();

        // execute update
        loop {
            match update_statement.next() {
                Ok(SqliteState::Done) => break,
                Ok(SqliteState::Row) => continue,
                Err(e) => return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": format!("Failed to execute update: {}", e)}))
                ).into_response()
            }
        }

        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "URL updated successfully",
                "short_code": short_code,
                "new_original_url": update_request.new_original_url,
            }))
        ).into_response()
    }

 
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": format!("Short code '{}' not found", short_code)}))
    ).into_response()
}