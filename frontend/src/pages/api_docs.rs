use yew::prelude::*;
use crate::components::{navbar::Navbar, footer::Footer};

#[function_component(ApiDocs)]
pub fn api_docs() -> Html {
    html! {
    <>
        <Navbar />
        <main>
            // Hero
            <section class="api-hero py-5">
                <div class="container text-center">
                    <h1 class="fw-bold">{ "ForgeURL API Documentation" }</h1>
                    <p class="mt-2 text-primary">
                        { "Simple, fast REST API built with Rust, Axum, Serde & SQLite" }
                    </p>
                </div>
            </section>
            <section class="py-5 bg-light">
                <div class="container">
                    // Base URL
                    <div class="mb-5">
                        <h4 class="fw-semibold">{ "Base URL" }</h4>
                        <pre class="code-box">{ "https://forgeurl-production.up.railway.app" }</pre>
                    </div>
                    // Authentication
                    <div class="mb-5">
                        <h4 class="fw-semibold">{ "Authentication" }</h4>
                        <p class="text-muted">
                            { "No authentication required. This API is open for public use." }
                        </p>
                    </div>
                    // URL Validation
                    <div class="mb-5">
                        <h4 class="fw-semibold">{ "URL Validation" }</h4>
                        <p class="text-muted">{ "All submitted URLs must meet these requirements:" }</p>
                        <ul class="text-muted">
                            <li>{ "URL must not be empty" }</li>
                            <li>{ "URL must start with http:// or https://" }</li>
                            <li>{ "Submitting the same URL twice returns the existing short code" }</li>
                        </ul>
                    </div>
                    // POST /
                    <div class="api-card mb-4 shadow-lg">
                        <div>
                            <span class="badge bg-primary">{ "POST" }</span>
                            <code>{ " https://forgeurl-production.up.railway.app/" }</code>
                        </div>
                        <p class="mt-3 text-muted">
                            { "Shorten a long URL. If the URL was already shortened, returns the existing record." }
                        </p>
                        <h6>{ "Request Body" }</h6>
                        <pre class="code-box">
                            { r#"{
   "original_url": "https://www.example.com/your/long/url"
}"# }
                        </pre>
                        <h6>{ "Response — 200 OK" }</h6>
                        <pre class="code-box">
                            { r#"{
    "short_code":   "QYFBTo",
    "short_url":    "https://forgeurl-production.up.railway.app/QYFBTo",
    "click_count":  0,
    "original_url": "https://www.example.com/your/long/url",
    "created_at":   "2026-03-24 10:00:00"
}"# }
                        </pre>
                        <h6>{ "Error Responses" }</h6>
                        <pre class="code-box">
                            { r#"400 Bad Request
  { "error": "URL cannot be empty" }
 
400 Bad Request
  { "error": "URL must start with http:// or https://" }
 
500 Internal Server Error
 {"error": "Database lock failed"}

500 Internal Server Error
 {"error": "Failed to prepare query"}

500 Internal Server Error
 {"error": "Failed to generate unique short code"}

500 Internal Server Error
 {"error": "Failed to execute query"}"#}
                        </pre>
                    </div>


                    // GET /{short_code}
                    <div class="api-card mb-4 shadow-lg">
                        <div>
                            <span class="badge bg-success">{ "GET" }</span>
                            <code>{ " https://forgeurl-production.up.railway.app/{short_code}" }</code>
                        </div>
                        <p class="mt-3 text-muted">
                            { "Redirects to the original URL and increments the click count by 1. This is a browser redirect — not a JSON response." }
                        </p>
                        <h6>{ "URL Parameters" }</h6>
                        <pre class="code-box">{ "short_code — The 6 character short code (e.g. QYFBTo)" }</pre>
                        <h6>{ "Response — 302 Found" }</h6>
                        <pre class="code-box">{ "Redirects directly to the original URL" }</pre>
                        <h6>{ "Error Responses" }</h6>
                        <pre class="code-box">
                           { r#"404 Not Found
  { "error": "Short code 'QYFBTo' not found" }
 
500 Internal Server Error
 {"error": "Database lock failed"}

500 Internal Server Error
 {"error": "Failed to prepare query"}

500 Internal Server Error
 {"error": "Failed to update click count"}"# }
                        </pre>
                    </div>


                    // PUT /{short_code}
                    <div class="api-card mb-4 shadow-lg">
                        <div>
                            <span class="badge bg-warning text-dark">{ "PUT" }</span>
                            <code>{ " https://forgeurl-production.up.railway.app/{short_code}" }</code>
                        </div>
                        <p class="mt-3 text-muted">
                            { "Updates the original URL that a short code points to. The short code itself does not change." }
                        </p>
                        <h6>{ "URL Parameters" }</h6>
                        <pre class="code-box">{ "short_code — The 6 character short code (e.g. QYFBTo)" }</pre>
                        <h6>{ "Request Body" }</h6>
                        <pre class="code-box">
                           { r#"{
    "new_original_url": "https://www.new-destination.com/page"
}"# }
                        </pre>
                        <h6>{ "Response — 200 OK" }</h6>
                        <pre class="code-box">
                           { r#"{
    "message": "URL updated successfully",
    "short_code":   "QYFBTo",
    "new_original_url": "https://www.new-destination.com/page"
}"# }
                        </pre>
                        <h6>{ "Error Responses" }</h6>
                        <pre class="code-box">
                           { r#"400 Bad Request
  { "error": "URL cannot be empty" }

400 Bad Request
   {"error": "URL must start with http:// or https://"}

404 Not Found
  { "error": "Short code 'QYFBTo' not found" }

500 Internal Server Error
 {"error": "Database lock failed"}

500 Internal Server Error
 {"error": "Failed to prepare query"}

500 Internal Server Error
 {"error": "Failed to execute update"}"# }
                        </pre>
                    </div>

                    // DELETE /{short_code}
                    <div class="api-card mb-4 shadow-lg">
                        <div>
                            <span class="badge bg-danger">{ "DELETE" }</span>
                            <code>{ " https://forgeurl-production.up.railway.app/{short_code}" }</code>
                        </div>
                        <p class="mt-3 text-muted">
                            { "Permanently deletes a short URL record from the database." }
                        </p>
                        <h6>{ "URL Parameters" }</h6>
                        <pre class="code-box">{ "short_code — The 6 character short code (e.g. QYFBTo)" }</pre>
                        <h6>{ "Response — 200 OK" }</h6>
                        <pre class="code-box">
                            { r#"{
    "message": "URL deleted successfully",
    "deleted_short_code": short_code,
    "deleted_original_url": original_url,
}"# }
                        </pre>
                        <h6>{ "Error Responses" }</h6>
                        <pre class="code-box">
                            { r#"404 Not Found
  { "error": "Short code 'QYFBTo' not found" }

500 Internal Server Error
 {"error": "Database lock failed"}

500 Internal Server Error
 {"error": "Failed to prepare query"}

500 Internal Server Error
 {"error": "Failed to execute delete"}"# }
                        </pre>
                    </div>


                    // GET /search
                    <div class="api-card mb-4 shadow-lg">
                        <div>
                            <span class="badge bg-success">{ "GET" }</span>
                            <code>{ " https://forgeurl-production.up.railway.app/search?search_string={term}" }</code>
                        </div>
                        <p class="mt-3 text-muted">
                            { "Searches through all stored URLs by keyword. Matches against both the short code and the original URL." }
                        </p>
                        <h6>{ "Query Parameters" }</h6>
                        <pre class="code-box">{ "search_string — The keyword to search for" }</pre>
                        <h6>{ "Response — 200 OK" }</h6>
                        <pre class="code-box">
                            { r#"[
    {
        "short_code":   "QYFBTo",
        "short_url":    "https://forgeurl-production.up.railway.app/QYFBTo",
        "click_count":  4,
        "original_url": "https://www.google.com/search?q=rust",
        "created_at":   "2026-03-24 10:00:00"
    }
]
// Returns empty array if no results found:
[]"# }
                        </pre>
                        <h6>{ "Error Responses" }</h6>
                        <pre class="code-box">
                            { r#"400 Bad Request
    { "error": "Search term cannot be empty" }

404 Not Found
 { "error": "Short code 'QYFBTo' not found" }

500 Internal Server Error
 {"error": "Database lock failed"}

500 Internal Server Error
 {"error": "Failed to prepare query"}

"# }
                                                        </pre>
                    </div>


                    // GET /analytics
                    <div class="api-card mb-4 shadow-lg">
                        <div>
                            <span class="badge bg-success">{ "GET" }</span>
                            <code>{ " https://forgeurl-production.up.railway.app/analytics" }</code>
                        </div>
                        <p class="mt-3 text-muted">
                            { "Returns overall analytics including total counts, most popular links, and recently created links." }
                        </p>
                        <h6>{ "Response — 200 OK" }</h6>
                        <pre class="code-box">
                            { r#"{
    "total_links":  5,
    "total_clicks": 24,
    "most_popular_link": {
        "short_code":   "QYFBTo",
        "short_url":    "https://forgeurl-production.up.railway.app/QYFBTo",
        "click_count":  12,
        "original_url": "https://www.google.com",
        "created_at":   "2026-03-24 10:00:00"
    },
    "popular_links": [ ...GetUrlResponse[] ],
    "recent_links":  [ ...GetUrlResponse[] ]
}


                            
// most_popular_link is null when no URLs exist:
{
    "total_links":       0,
    "total_clicks":      0,
    "most_popular_link": null,
    "popular_links":     [],
    "recent_links":      []
}"# }
                        </pre>
                        <h6>{ "Error Responses" }</h6>
                        <pre class="code-box">{ r#"500 Internal Server Error
 {"error": "Database lock failed"}

500 Internal Server Error
 {"error": "Failed to prepare query"}

"# }</pre>
                    </div>
                    // Data Models
                    <div class="mb-5 mt-5">
                        <h4 class="fw-semibold">{ "Data Models" }</h4>
                    </div>
                    <div class="api-card mb-4 shadow-lg">
                        <h6>{ "ShortenRequest" }</h6>
                        <p class="text-muted small">{ "Request body for POST /" }</p>
                        <pre class="code-box">
                               { r#"{
    "original_url": String   // required — the full URL to shorten
}"# }
                        </pre>
                    </div>
                    <div class="api-card mb-4 shadow-lg">
                        <h6>{ "UpdateRequest" }</h6>
                        <p class="text-muted small">{ "Request body for PUT /" }</p>
                        <pre class="code-box">
                               { r#"{
    "new_original_url": String   // required — the full URL to shorten
}"# }
                        </pre>
                    </div>
                    <div class="api-card mb-4 shadow-lg">
                        <h6>{ "GetUrlResponse" }</h6>
                        <p class="text-muted small">{ "Returned by POST /, GET /search" }</p>
                        <pre class="code-box">
                              { r#"{
    "short_code":   String,  // 6 character unique identifier
    "short_url":    String,  // full short URL including base URL
    "click_count":  Integer, // number of times the link was visited
    "original_url": String,  // the original long URL
    "created_at":   String   // creation timestamp UTC
}"# }
                        </pre>
                    </div>
                    <div class="api-card shadow-lg">
                        <h6>{ "AnalyticsResponse" }</h6>
                        <p class="text-muted small">{ "Returned by GET /analytics" }</p>
                        <pre class="code-box">
                             { r#"{
    "total_links":        Integer,          // total stored URLs
    "total_clicks":       Integer,          // total clicks across all URLs
    "most_popular_link":  GetUrlResponse?,  // most clicked URL (nullable)
    "popular_links":      GetUrlResponse[], // sorted by click count
    "recent_links":       GetUrlResponse[]  // sorted by creation date
}"# }
                        </pre>
                    </div>
                </div>
            </section>
        </main>
        <Footer />
    </>
    }
}