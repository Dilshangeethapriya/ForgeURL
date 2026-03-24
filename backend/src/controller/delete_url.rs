use axum::{Json, extract::{State, Path}, http::StatusCode, response::IntoResponse};
use sqlite::State as SqliteState;
use crate::Db;

pub async fn delete_url(Path(short_code): Path<String>,State(db): State<Db>) -> impl IntoResponse {

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
        let original_url: String = statement.read(2).unwrap();
        // if short_code exists delete it
        let delete_query = "DELETE FROM shorten_url WHERE short_code = ?";
        let mut delete_statement = match connection.prepare(delete_query) {
            Ok(stmt) => stmt,
            Err(e) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to prepare delete: {}", e)}))
            ).into_response()
        };

        delete_statement.bind((1, short_code.as_str())).unwrap();

        // execute delete query
        loop {
            match delete_statement.next() {
                Ok(SqliteState::Done) => break,
                Ok(SqliteState::Row) => continue,
                Err(e) => return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": format!("Failed to execute delete: {}", e)}))
                ).into_response()
            }
        }

        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "URL deleted successfully",
                "deleted_short_code": short_code,
                "deleted_original_url": original_url,
            }))
        ).into_response()
    }

 
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": format!("Short code '{}' not found", short_code)}))
    ).into_response()
}