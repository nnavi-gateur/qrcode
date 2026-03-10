#[macro_use]
extern crate rocket;

use qrcodegen::svg::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde_json;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[get("/qrcode/SVG?<content>&<level>")]
fn qr_svg(content: String, level: i32) -> Json<Response> {
    println!("Received request for content: {}, level: {}", content, level);
    Json(Response {
        message: create_svg(&create_qr_code(&content, level)).to_string(),
    })
}
#[get("/qrcode/JPG?<content>&<level>")]
fn qr_jpg(content: String, level: i32) -> Json<Response> {
    println!("Received request for content: {}, level: {}", content, level);
    Json(Response {
        message: create_jpg(&create_qr_code(&content, level)).to_string(),
    })
}

/// Request body received from the frontend for URL shortening.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ShortenRequest {
    url: String,
}

/// Response returned to the frontend after shortening.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ShortenResponse {
    short_url: String,
}

/// Config read from environment variables by Rocket.
struct AppConfig {
    rs_short_url: String,
    rs_instance_hostname: String,
}

/// POST /shorten
/// Proxies the URL to rs-short's /api/shorten endpoint and returns the short URL.
/// Requires the RS_SHORT_URL environment variable to be set.
#[post("/shorten", data = "<body>")]
async fn shorten(
    body: Json<ShortenRequest>,
    cfg: &State<AppConfig>,
) -> Result<Json<ShortenResponse>, rocket::response::status::Custom<String>> {
    use rocket::http::Status;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap_or_default();

    let payload = serde_json::json!({
        "url_to": body.url,
    });

    let resp = client
        .post(format!("{}/api/shorten", cfg.rs_short_url))
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            rocket::response::status::Custom(
                Status::BadGateway,
                format!("rs-short unreachable: {}", e),
            )
        })?;

    if !resp.status().is_success() {
        let status_code = resp.status().as_u16();
        let body_text = resp.text().await.unwrap_or_default();
        return Err(rocket::response::status::Custom(
            Status::new(status_code),
            format!("rs-short error: {}", body_text),
        ));
    }

    let data: serde_json::Value = resp.json().await.map_err(|e| {
        rocket::response::status::Custom(
            Status::InternalServerError,
            format!("failed to parse rs-short response: {}", e),
        )
    })?;

    // Extract just the slug from whatever URL rs-short returned, then
    // rebuild the final URL using the configured public hostname.
    let raw = data["short_url"].as_str().unwrap_or("");
    let slug = raw.rsplit('/').next().unwrap_or("");
    let short_url = format!("{}/{}", cfg.rs_instance_hostname, slug);

    Ok(Json(ShortenResponse { short_url }))
}

/// OPTIONS /shorten
/// Handles CORS preflight requests
#[options("/shorten")]
fn shorten_options() -> &'static str {
    ""
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();
    
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        ..Default::default()
    }
    .to_cors()
    .expect("CORS error");

    let rs_short_url = std::env::var("RS_SHORT_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    let rs_instance_hostname = std::env::var("RS_INSTANCE_HOSTNAME")
        .unwrap_or_else(|_| "https://s.rezel.net".to_string());

    rocket::build()
        .attach(cors)
        .manage(AppConfig {
            rs_short_url,
            rs_instance_hostname,
        })
        .mount("/api", routes![qr_svg, qr_jpg, shorten, shorten_options])
}
