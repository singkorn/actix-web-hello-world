use actix_web::{get, web, web::ServiceConfig, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct WeatherQuery {
    city: String,
}

#[derive(Serialize)]
struct Weather {
    city: String,
    temperature: f64,
    description: String,
    humidity: u8,
}

#[derive(Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<GeocodingResult>>,
}

#[derive(Deserialize)]
struct GeocodingResult {
    latitude: f64,
    longitude: f64,
    name: String,
}

#[derive(Deserialize)]
struct OpenMeteoWeatherResponse {
    current_weather: CurrentWeather,
}

#[derive(Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/weather")]
async fn weather(query: web::Query<WeatherQuery>) -> HttpResponse {
    let city = &query.city;
    
    // 1. Get coordinates for the city
    let geo_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city
    );

    let client = reqwest::Client::new();
    
    let geo_resp = match client.get(&geo_url).send().await {
        Ok(resp) => resp,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to fetch geocoding data"),
    };

    let geo_data: GeocodingResponse = match geo_resp.json().await {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse geocoding data"),
    };

    let result = match geo_data.results {
        Some(results) if !results.is_empty() => &results[0],
        _ => return HttpResponse::NotFound().body(format!("City '{}' not found", city)),
    };

    // 2. Get weather for coordinates
    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        result.latitude, result.longitude
    );

    let weather_resp = match client.get(&weather_url).send().await {
        Ok(resp) => resp,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to fetch weather data"),
    };

    let weather_data: OpenMeteoWeatherResponse = match weather_resp.json().await {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse weather data"),
    };

    let weather = Weather {
        city: result.name.clone(),
        temperature: weather_data.current_weather.temperature,
        description: format!("Wind speed: {} km/h", weather_data.current_weather.windspeed), // Open-Meteo simple API doesn't give text description easily
        humidity: 0, // Not available in simple current_weather response
    };

    HttpResponse::Ok().json(weather)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
           .service(weather);
    };

    Ok(config.into())
}
