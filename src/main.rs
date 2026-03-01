#[macro_use]
extern crate rocket;

use qrcodegen::svg::*;
use rocket::serde::{json::Json, Serialize};
use rocket_cors::{AllowedOrigins, CorsOptions};

#[derive(Serialize)]
struct Response {
    message: String,
}

#[get("/qrcode")]
fn hello() -> Json<Response> {
    Json(Response {
        message: create_svg(&create_qr_code("Hello World", 2)).to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default().to_cors().expect("CORS error");

    rocket::build().attach(cors).mount("/", routes![hello])
}
