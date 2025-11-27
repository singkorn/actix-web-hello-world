use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub username: String,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub token: String,
    pub username: String,
}

#[post("/tokens")]
pub async fn create_token(
    data: web::Json<CreateTokenRequest>,
    persist: web::Data<PersistInstance>,
) -> HttpResponse {
    let token = Uuid::new_v4().to_string();
    let username = &data.username;

    // Save token -> username mapping
    // In a real app, we might want username -> [tokens] or similar, 
    // but for auth validation we just need to know if the token exists.
    // Saving the username as the value allows us to potentially retrieve it later.
    let _ = persist.save(&token, username);

    HttpResponse::Ok().json(CreateTokenResponse {
        token,
        username: username.clone(),
    })
}
