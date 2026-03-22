use axum::{Json, extract::{Query, State}, http::StatusCode, response::IntoResponse};
use shared::types::{Db, GetUrlResponse, SearchQuery};
use sqlite::{ State as SqliteState};


pub async fn search_url(State(db): State<Db>,Query(search_query): Query<SearchQuery>) -> impl IntoResponse {
  let connection = match db.lock(){
        Ok(conn) => conn,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database lock failed"}))
        ).into_response()
    };
    
      // validate search term
    if search_query.search_string.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Search term cannot be empty"}))
        ).into_response()
    }

    let select_query = "SELECT * FROM shorten_url WHERE original_url LIKE ?";
    let mut statement = match connection.prepare(select_query) {
    Ok(stmt) => stmt,
    Err(e) => return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": format!("Failed to prepare query: {}", e)}))
    ).into_response()
    };
    let search_term = format!("%{}%", search_query.search_string);
    statement.bind((1, search_term.as_str())).unwrap();
  
     let mut results = Vec::new();
     while let Ok(SqliteState::Row) = statement.next() {
        let short_code: String = statement.read(1).unwrap();
        let original_url: String = statement.read(2).unwrap();
        let created_at: String = statement.read(3).unwrap();
        let click_count: i64 = statement.read(4).unwrap();
        let short_url = format!("http://localhost:7878/{}", short_code);

        results.push(GetUrlResponse {
            short_url,
            short_code,
            original_url,
            created_at,
            click_count, 
        });
     }

        if !results.is_empty() {
            return (StatusCode::OK, Json(results)).into_response()
        }
        
        (
         StatusCode::NOT_FOUND,
         Json(serde_json::json!({"error": format!("Original URL '{}' not found", search_query.search_string)}))
        ).into_response()

}