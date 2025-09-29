use crate::Dasharr;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web::Data};
use futures::future::{Ready, ready};

pub struct ApiKey(pub String);

impl FromRequest for ApiKey {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let api_key = req
            .headers()
            .get("api-key")
            .and_then(|value| value.to_str().ok())
            .map(|s| s.to_owned());

        match api_key {
            Some(key) => ready(Ok(ApiKey(key))),
            None => ready(Err(actix_web::error::ErrorUnauthorized(
                "authentication error: missing API key",
            ))),
        }
    }
}

pub async fn authenticate_user(
    req: ServiceRequest,
    api_key: ApiKey,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let arc = req.app_data::<Data<Dasharr>>().expect("app data set");

    if api_key.0 != arc.env.api_key {
        Err((
            ErrorUnauthorized("authentication error: invalid API key"),
            req,
        ))
    } else {
        Ok(req)
    }
}
