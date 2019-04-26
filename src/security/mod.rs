
use iron::{Request, IronResult, Response};
use jsonwebtoken::TokenData;
use iron::status;

use crate::security::jwt::{from_token, Claims};

pub mod jwt;

pub static SALT: usize = 12;

pub fn get_claims(request: &Request) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let auth_header = request.headers.iter().find(|x| x.name() == "Authorization");
    match auth_header {
        Some(header) => {
            from_token(&header.value_string())
        },
        None => {
            error!("Missing Authorization header!");
            Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))
        }
    }
}

pub fn get_claims_with_handler(request: &Request) -> Result<Claims, IronResult<Response>> {
    match get_claims(request) {
        Ok(data) => Ok(data.claims),
        Err(e) => {
            error!("TOKEN INVALID! {}", e);
            Err(Ok(Response::with(status::Unauthorized)))
        }
    }
}