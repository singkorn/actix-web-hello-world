use actix_web::{dev::ServiceRequest, Error, error::ErrorUnauthorized, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use shuttle_persist::PersistInstance;

pub async fn validator(req: ServiceRequest, auth: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let persist = req.app_data::<web::Data<PersistInstance>>().unwrap();
    let token = auth.token();
    
    // Check if the token exists as a key. We try to load it as a String.
    // Ideally, the value would be user info, but for now we just check existence.
    // If load returns Ok, the key exists.
    let exists = persist.load::<String>(token).is_ok();

    if exists {
        Ok(req)
    } else {
        Err((ErrorUnauthorized("Invalid token"), req))
    }
}
