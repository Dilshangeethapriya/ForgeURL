use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize)]

pub struct ShortenRequest  {
    pub original_url: String,
}

#[derive(Debug,Serialize,Deserialize)]

pub struct UpdateRequest  {
    pub new_original_url: String,
}



// #[derive(Debug,Serialize,Deserialize, Clone)]
// pub struct ShortenResponse {
//     pub short_code: String,
//     pub short_url: String,
//     pub click_count: i64,
//     pub original_url: String,
// }


#[derive(Debug,Serialize,Deserialize, Clone, PartialEq)]
pub struct GetUrlResponse {
    pub short_code: String,
    pub short_url: String,
    pub click_count: i64,
    pub original_url: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub search_string : String,  
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalyticsResponse {
    pub total_links: i64,
    pub total_clicks: i64,
    pub most_popular_link: Option<GetUrlResponse>,
    pub popular_links: Vec<GetUrlResponse>,
    pub recent_links: Vec<GetUrlResponse>,
}