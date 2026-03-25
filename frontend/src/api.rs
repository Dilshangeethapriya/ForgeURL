#[cfg(debug_assertions)]
pub const API_BASE: &str = "http://localhost:7878";

#[cfg(not(debug_assertions))]
pub const API_BASE: &str = "https://forgeurl-production.up.railway.app";