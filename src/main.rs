#[macro_use]
extern crate rocket;

use qrcodegen::svg::*;
use rocket::serde::{json::Json, Serialize};
use rocket_cors::{AllowedOrigins, CorsOptions};

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


#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default().to_cors().expect("CORS error");

    rocket::build().attach(cors).mount("/", routes![qr_svg, qr_jpg])
}
