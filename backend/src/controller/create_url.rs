use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use shared::types::{GetUrlResponse, ShortenRequest};
use rand::{distributions::Alphanumeric, Rng};
use sqlite::{Connection, State as SqliteState};
use crate::Db;
// generating a  random 6 charactor(alphanumeric) string for short_code
fn generate_short_code() -> String { 
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}



// Cheking the short code uniqueness
fn is_short_code_taken(connection: &Connection, short_code: &str) -> bool {
    let query = "SELECT COUNT(*) FROM shorten_url WHERE short_code = ?";
    
    let mut statement = match connection.prepare(query) {
        Ok(stmt) => stmt,
        Err(_) => return true, 
    };

    if statement.bind((1, short_code)).is_err() {
        return true; 
    }

    if let Ok(SqliteState::Row) = statement.next() {
        let count: i64 = statement.read(0).unwrap_or(0);
        return count > 0;
    }

    false
}



// repete generating until a unique shrt-code is found
fn generate_unique_short_code(connection: &Connection) -> Option<String> {
    let max_attempts = 10;  // give up after 10 tries
    
    for _ in 0..max_attempts {
        let short_code = generate_short_code();
        
        if !is_short_code_taken(connection, &short_code) {
            return Some(short_code);
        }
    }
    
    None
}




pub async fn create_url(State(db): State<Db>, Json(shorten_request) : Json<ShortenRequest>) -> impl IntoResponse {

       // validating the URL
    if shorten_request.original_url.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "URL cannot be empty"}))
        ).into_response()
    }

    if !shorten_request.original_url.starts_with("http://") && 
       !shorten_request.original_url.starts_with("https://") {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "URL must start with http:// or https://"}))
        ).into_response()
    }

  // db connection
  let connection = match db.lock(){
        Ok(conn) => conn,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database lock failed"}))
        ).into_response()
    };


    
  let table_query = "CREATE TABLE IF NOT EXISTS shorten_url(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    short_code   TEXT    NOT NULL UNIQUE,   
    original_url TEXT    NOT NULL,
    created_at   TEXT    NOT NULL DEFAULT (datetime('now')),
    click_count  INTEGER NOT NULL DEFAULT 0
    )";

    connection.execute(table_query).unwrap();




    // check if original_url already exists
    let check_query = "SELECT * FROM shorten_url WHERE original_url = ?";
    let mut statement = match connection.prepare(check_query) {
        Ok(stmt) => stmt,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to prepare query: {}", e)}))
        ).into_response()
    };
    statement.bind((1, shorten_request.original_url.as_str())).unwrap();
    
    // if exists return existing record
    if let Ok(SqliteState::Row) = statement.next() {
        let short_code: String = statement.read(1).unwrap();
        let short_url = format!("http://localhost:7878/{}", short_code);
    
        let response = GetUrlResponse {
            short_url,
            short_code,
            original_url: statement.read(2).unwrap(),
            created_at: statement.read(3).unwrap(),
            click_count: statement.read(4).unwrap(),
        };
    
        return (StatusCode::OK, Json(response)).into_response()
    }




    let short_code = match generate_unique_short_code(&connection) {
    Some(code) => code,
    None => return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": "Failed to generate unique short code"}))
    ).into_response()
    };

    let insert_query = "INSERT INTO shorten_url (original_url, short_code) VALUES (? , ?)";
    let mut statement = connection.prepare(insert_query).unwrap();
    statement.bind((1, shorten_request.original_url.as_str())).unwrap();
    statement.bind((2, short_code.as_str())).unwrap();

    loop {
    match statement.next(){
         Ok(SqliteState::Done) => break,
            Ok(SqliteState::Row) => continue,
            Err(e) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to execute insert: {}", e)}))
            ).into_response()   
    }
    }

    let short_url = format!("http://localhost:7878/{}", short_code);


    let response = GetUrlResponse {
        short_code,
        short_url,
        original_url: shorten_request.original_url,
        click_count: 0,
        created_at: String::from(""),
    };

    (StatusCode::CREATED, Json(response)).into_response()
}