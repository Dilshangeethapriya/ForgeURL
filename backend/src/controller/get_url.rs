use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse, response::Redirect};
use sqlite::{ State as SqliteState};
use crate::Db;


pub async fn get_url(Path(short_code): Path<String>, State(db): State<Db>) -> impl IntoResponse {
  let connection = match db.lock(){
        Ok(conn) => conn,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database lock failed"}))
        ).into_response()
    };
    

    let select_query = "SELECT * FROM shorten_url WHERE short_code = ?";
    let mut statement = match connection.prepare(select_query) {
    Ok(stmt) => stmt,
    Err(e) => return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": format!("Failed to prepare query: {}", e)}))
    ).into_response()
    };
    statement.bind((1, short_code.as_str())).unwrap();
     let original_url: String;
     if let Ok(SqliteState::Row) = statement.next() {
        original_url = statement.read(2).unwrap();
        // increase click count
        let update_query = "UPDATE shorten_url SET click_count = click_count + 1 WHERE short_code = ?";
        let mut update_statement = match connection.prepare(update_query) {
            Ok(stmt) => stmt,
            Err(e) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to prepare update: {}", e)}))
            ).into_response()
        };
        update_statement.bind((1, short_code.as_str())).unwrap();

        loop {
            match update_statement.next() {
                Ok(SqliteState::Done) => break,
                Ok(SqliteState::Row) => continue,
                Err(e) => return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": format!("Failed to update click count: {}", e)}))
                ).into_response()
            }
        }


        return Redirect::temporary(&original_url).into_response()
    } 
        (
         StatusCode::NOT_FOUND,
         Json(serde_json::json!({"error": format!("Short code '{}' not found", short_code)}))
        ).into_response()

}