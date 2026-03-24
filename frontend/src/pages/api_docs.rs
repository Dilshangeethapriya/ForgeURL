use yew::prelude::*;
use crate::components::{navbar::Navbar, footer::Footer};

#[function_component(ApiDocs)]
pub fn api_docs() -> Html {
    html! {
        <>
            <Navbar />
            <main>
                <section class="api-hero py-5">
                    <div class="container text-center">
                        <h1 class="fw-bold">{ "ForgeURL API Documentation" }</h1>
                        <p class="mt-2">
                            { "Simple, fast REST API built with Rust, Axum, Serde & SQLite" }
                        </p>
                    </div>
                </section>

                <section class="py-5 bg-light">
                    <div class="container">

                        <div class="mb-5">
                            <h4 class="fw-semibold">{ "Base URL" }</h4>
                            <pre class="code-box">{ "http://127.0.0.1:7878" }</pre>
                        </div>

                        <div class="mb-5">
                            <h4 class="fw-semibold">{ "Authentication" }</h4>
                            <p class="text-muted">
                                { "You don't need authentication to use this demo." }
                            </p>
                        </div>

                        // POST /shorten
                        <div class="api-card mb-4 shadow-lg">
                            <div>
                                <span class="badge bg-primary">{ "POST" }</span>
                                <code>{ " /" }</code>
                            </div>
                            <p class="mt-3 text-muted">{ "Shorten a new URL." }</p>
                            <h6>{ "Request Body" }</h6>
                            <pre class="code-box">
{ r#"{
    "original_url": "https://www.example.com/very/long/url"
}"# }
                            </pre>
                            <h6>{ "Response (201 Created)" }</h6>
                            <pre class="code-box">
{ r#"{
    "short_code": "abc123",
    "short_url": "http://127.0.0.1:7878/abc123",
    "original_url": "https://www.example.com/very/long/url",
    "click_count": 0,
    "created_at": "2026-01-01 10:00:00"
}"# }
                            </pre>
                        </div>

                        // GET /{short_code}
                        <div class="api-card mb-4 shadow-lg">
                            <div>
                                <span class="badge bg-success">{ "GET" }</span>
                                <code>{ " /{short_code}" }</code>
                            </div>
                            <p class="mt-3 text-muted">
                                { "Redirects to the original URL and increments click count." }
                            </p>
                            <h6>{ "Response (307 Redirect)" }</h6>
                            <pre class="code-box">{ "Redirects to original URL" }</pre>
                            <h6>{ "Not Found (404)" }</h6>
                            <pre class="code-box">
{ r#"{
    "error": "Short code 'abc123' not found"
}"# }
                            </pre>
                        </div>

                        // PUT /{short_code}
                        <div class="api-card mb-4 shadow-lg">
                            <div>
                                <span class="badge bg-warning text-dark">{ "PUT" }</span>
                                <code>{ " /{short_code}" }</code>
                            </div>
                            <p class="mt-3 text-muted">
                                { "Update the original URL for an existing short code." }
                            </p>
                            <h6>{ "Request Body" }</h6>
                            <pre class="code-box">
{ r#"{
    "original_url": "https://www.updated-url.com"
}"# }
                            </pre>
                            <h6>{ "Response (200 OK)" }</h6>
                            <pre class="code-box">
{ r#"{
    "message": "URL updated successfully",
    "short_code": "abc123",
    "new_original_url": "https://www.updated-url.com"
}"# }
                            </pre>
                        </div>

                        // DELETE /{short_code}
                        <div class="api-card mb-4 shadow-lg">
                            <div>
                                <span class="badge bg-danger">{ "DELETE" }</span>
                                <code>{ " /{short_code}" }</code>
                            </div>
                            <p class="mt-3 text-muted">
                                { "Delete a shortened URL." }
                            </p>
                            <h6>{ "Response (200 OK)" }</h6>
                            <pre class="code-box">
{ r#"{
    "message": "URL deleted successfully",
    "deleted_short_code": "abc123",
    "deleted_original_url": "https://www.example.com"
}"# }
                            </pre>
                        </div>

                        // GET /search
                        <div class="api-card mb-4 shadow-lg">
                            <div>
                                <span class="badge bg-success">{ "GET" }</span>
                                <code>{ " /search?q={term}" }</code>
                            </div>
                            <p class="mt-3 text-muted">
                                { "Search URLs by keyword." }
                            </p>
                            <h6>{ "Example" }</h6>
                            <pre class="code-box">{ "GET /search?q=youtube" }</pre>
                            <h6>{ "Response (200 OK)" }</h6>
                            <pre class="code-box">
{ r#"[
    {
        "short_code": "abc123",
        "short_url": "http://127.0.0.1:7878/abc123",
        "original_url": "https://www.youtube.com/watch?v=...",
        "click_count": 42,
        "created_at": "2026-01-01 10:00:00"
    }
]"# }
                            </pre>
                        </div>

                        // GET /analytics
                        <div class="api-card shadow-lg">
                            <div>
                                <span class="badge bg-success">{ "GET" }</span>
                                <code>{ " /analytics" }</code>
                            </div>
                            <p class="mt-3 text-muted">
                                { "Get full analytics data." }
                            </p>
                            <h6>{ "Response (200 OK)" }</h6>
                            <pre class="code-box">
{ r#"{
    "total_links": 150,
    "total_clicks": 3420,
    "most_popular_link": {
        "short_code": "abc123",
        "short_url": "http://127.0.0.1:7878/abc123",
        "original_url": "https://youtube.com",
        "click_count": 342,
        "created_at": "2026-01-01 10:00:00"
    },
    "popular_links": [...],
    "recent_links": [...]
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