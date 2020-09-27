use crate::errors::ServiceError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use actix_web::{FromRequest, HttpRequest, dev};
use std::error::Error;
use crate::user_handlers::AuthUser;
use bcrypt::{DEFAULT_COST, hash, verify};
use hex;
use csrf_token::CsrfTokenGenerator;


pub struct LightUser {
    pub email: String,

}

pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
    /*
    Consumes a token as a string, and returns a boolean result of
    the validation, or returns a service error if something went wrong.
    */
    let authority = std::env::var("Authority").expect("Authority must be set");
    let jwks = fetch_jwks(&format!("{}{}", authority.as_str(), ".well-known/jwks.json"))
        .expect("Failed to fetch JWKS.");
    
    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("Failed to decode."),
        Err(_) => return Err(ServiceError::JWKSFetchError)
    };
    
    let jwk = jwks.find(&kid).expect("Specified key not found");

    let res = validate(token, jwk, validations);

    Ok(res.is_ok())
}


fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri)?;
    let val = res.json::<JWKS>()?;
    return Ok(val)
}

pub fn login(auth: web::Json<AuthUser>, db: web::Data<Pool>, generator)