use std::sync::{Arc, Mutex};
use axum::{Router, routing::{get, post, put, delete}};
use sqlite::Connection;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method; 



pub type Db = Arc<Mutex<Connection>>;

use crate::controller::
    {
      create_url::create_url,
      delete_url::delete_url,
      get_analytics:: get_analytics,
      get_url::get_url,
      search_url::search_url,
      update_url::update_url
    };
mod controller;

#[tokio::main]
async fn main() {

let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
    std::fs::create_dir_all("data").ok(); 
    "data/forgeurl.db".to_string()
});

let connection = sqlite::open(&db_path).unwrap();
println!("Database opened at: {}", db_path);
   let db: Db = Arc::new(Mutex::new(connection));

  let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any); 

   
   
   let app = Router::new().route("/", post(create_url))
                                  .route("/{short_code}", get(get_url))
                                  .route("/{short_code}", put(update_url))
                                  .route("/{short_code}", delete(delete_url))
                                  .route("/search", get(search_url))
                                  .route("/analytics", get(get_analytics))
                                  // TODO: add rate limiting before deployment
                                  .with_state(db)
                                  .layer(cors);

   let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();

   println!("Server running on http://localhost:7878");
   axum::serve(listener, app).await.unwrap();
}
