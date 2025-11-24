use actix_web::{get, web, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use serde::Serialize;

#[derive(Serialize)]
struct Weather {
    temperature: f64,
    description: String,
    humidity: u8,
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/weather")]
async fn weather() -> web::Json<Weather> {
    web::Json(Weather {
        temperature: 25.5,
        description: "Sunny".to_string(),
        humidity: 60,
    })
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
           .service(weather);
    };

    Ok(config.into())
}
