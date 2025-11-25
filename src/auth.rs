use actix_web::{dev::ServiceRequest, Error, error::ErrorUnauthorized};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(req: ServiceRequest, auth: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if auth.token() == "my-secret-token" {
        Ok(req)
    } else {
        Err((ErrorUnauthorized("Invalid token"), req))
    }
}
