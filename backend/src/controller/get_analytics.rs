use axum::{Json, extract::{State}, http::StatusCode, response::IntoResponse};
use shared::types::{GetUrlResponse, AnalyticsResponse};
use sqlite::{ State as SqliteState};
use crate::Db;

pub async fn get_analytics( State(db): State<Db>) -> impl IntoResponse {
  let connection = match db.lock(){
        Ok(conn) => conn,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database lock failed"}))
        ).into_response()
    };
    




    // get total shortened links
    let mut total_links: i64 = 0;
    let total_links_query = "SELECT COUNT(*) FROM shorten_url";
    let mut statement = match connection.prepare(total_links_query) {
    Ok(stmt) => stmt,
    Err(e) => return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": format!("Failed to prepare query: {}", e)}))
    ).into_response()
    };
  
     if let Ok(SqliteState::Row) = statement.next() {
        total_links = statement.read(0).unwrap();
    } 







     // get total click count
    let mut total_clicks: i64 = 0;
    let total_clicks_query = "SELECT SUM(click_count) FROM shorten_url";
    let mut statement = match connection.prepare(total_clicks_query) {
    Ok(stmt) => stmt,
    Err(e) => return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": format!("Failed to prepare query: {}", e)}))
    ).into_response()
    };
  
    
     if let Ok(SqliteState::Row) = statement.next() {
        total_clicks = statement.read(0).unwrap_or(0);
    } 






      // get most popular link
    let mut most_popular_link: Option<GetUrlResponse> = None;
    let most_popular_link_query = "SELECT * FROM shorten_url ORDER BY click_count DESC LIMIT 1";
    let mut statement = match connection.prepare(most_popular_link_query) {
    Ok(stmt) => stmt,
    Err(e) => return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": format!("Failed to prepare query: {}", e)}))
    ).into_response()
    };
  
     
     if let Ok(SqliteState::Row) = statement.next() {
        let short_code: String = statement.read(1).unwrap();
        let short_url = format!("http://localhost:7878/{}", short_code);
        most_popular_link = Some(GetUrlResponse {
            original_url: statement.read(2).unwrap(),
            click_count: statement.read(4).unwrap(),
            short_url: short_url,
            short_code,
            created_at: statement.read(3).unwrap(),
        });
    } 







    // get popular links
     let mut popular_links: Vec<GetUrlResponse> = Vec::new();
     let popular_links_query = "SELECT * FROM shorten_url ORDER BY click_count DESC LIMIT 5";
     let mut statement = match connection.prepare(popular_links_query) {
        Ok(stmt) => stmt,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to prepare query: {}", e)}))
        ).into_response()
     };

     while let Ok(SqliteState::Row) = statement.next() {
        let short_code: String = statement.read(1).unwrap();
        let short_url = format!("https://forgeurl-production.up.railway.app/{}", short_code);
        popular_links.push(GetUrlResponse {
            original_url: statement.read(2).unwrap(),
            click_count: statement.read(4).unwrap(),
            short_url: short_url,
            short_code,
            created_at: statement.read(3).unwrap(),
        });
     }






     // get recent links
        let mut recent_links: Vec<GetUrlResponse> = Vec::new();
        let recent_links_query = "SELECT * FROM shorten_url ORDER BY created_at DESC LIMIT 5";
        let mut statement = match connection.prepare(recent_links_query) {
           Ok(stmt) => stmt,
           Err(e) => return (
               StatusCode::INTERNAL_SERVER_ERROR,
               Json(serde_json::json!({"error": format!("Failed to prepare query: {}", e)}))
           ).into_response()
        };

        while let Ok(SqliteState::Row) = statement.next() {
           let short_code: String = statement.read(1).unwrap();
           let short_url = format!("https://forgeurl-production.up.railway.app/{}", short_code);
           recent_links.push(GetUrlResponse {
               original_url: statement.read(2).unwrap(),
               click_count: statement.read(4).unwrap(),
               short_url: short_url,
               short_code,
               created_at: statement.read(3).unwrap(),
           });
        }



        let analytics_response = AnalyticsResponse {
            total_links,
            total_clicks,
            most_popular_link,
            popular_links,
            recent_links,
        };
    

        (
         StatusCode::OK,
         Json(analytics_response)
        ).into_response()

}