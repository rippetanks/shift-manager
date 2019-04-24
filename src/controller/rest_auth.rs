
extern crate mime;
extern crate hex;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use serde::{Deserialize, Serialize};
use std::io::Read;
use sha3::{Digest, Sha3_512};

use crate::database::user_db::*;
use crate::security::jwt::*;
use crate::security::get_claims_with_handler;

#[derive(Serialize,Deserialize)]
struct LoginData {
    email: String,
    password: String
}

pub fn login(request: &mut Request) -> IronResult<Response> {
    trace!("auth -> login");
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let data: Result<LoginData, serde_json::error::Error> = serde_json::from_str(&payload);
    match data {
        Ok(data) => h_login(&data),
        Err(e) => {
            error!("{}", e);
            Ok(Response::with(status::BadRequest))
        }
    }
}

fn h_login(data: &LoginData) -> IronResult<Response> {
    // login logic
    let user = find_by_email(&data.email);
    match user {
        Ok(user) => {
            match user {
                Some(user) => {
                    if !user.enable {
                        warn!("The user {} is disabled!", user.id);
                        return Ok(Response::with(status::Forbidden));
                    }
                    else {
                        let mut hasher = Sha3_512::new();
                        let pwd = data.password.clone() + &user.salt;
                        hasher.input(pwd);
                        if user.password != hex::encode(hasher.result()) {
                            warn!("The user {} enter wrong password!", user.id);
                            return Ok(Response::with(status::Unauthorized));
                        }
                    }
                    let token = create(user.id);
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("application/json".parse::<Mime>().unwrap());
                    response.set_mut(serde_json::to_string(&token).unwrap());
                    Ok(response)
                },
                None => {
                    warn!("User not found!");
                    Ok(Response::with(status::NotFound))
                }
            }
        },
        Err(_e) => {
            error!("Server Error!");
            Ok(Response::with(status::InternalServerError))
        }
    }
}

pub fn logout(_request: &mut Request) -> IronResult<Response> {
    trace!("auth -> logout");
    Ok(Response::with(status::NoContent))
}

pub fn keepalive(request: &mut Request) -> IronResult<Response> {
    trace!("auth -> keepalive");
    match get_claims_with_handler(request) {
        Ok(mut claims) => {
            let token = renew(&mut claims);
            let mut response = Response::new();
            response.set_mut(status::Ok);
            response.set_mut("application/json".parse::<Mime>().unwrap());
            response.set_mut(serde_json::to_string(&token).unwrap());
            Ok(response)
        },
        Err(e) => e
    }
}