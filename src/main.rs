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

#[get("/qrcode/SVG/<content>/<level>")]
fn qr_svg(content: String, level: i32) -> Json<Response> {
    println!("Received request for content: {}, level: {}", content, level);
    Json(Response {
        message: create_svg(&create_qr_code(&content, level)).to_string(),
    })
}
#[get("/qrcode/JPG/<content>/<level>")]
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

    let client = reqwest::Client::new();

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

    let short_url = data["short_url"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(Json(ShortenResponse { short_url }))
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default().to_cors().expect("CORS error");

    let rs_short_url = std::env::var("RS_SHORT_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    rocket::build()
        .attach(cors)
        .manage(AppConfig {
            rs_short_url,
        })
        .mount("/", routes![qr_svg, qr_jpg, shorten])
}
