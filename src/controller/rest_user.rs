
extern crate mime;
extern crate hex;
extern crate rand;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use std::io::Read;
use serde::{Deserialize, Serialize};
use sha3::{Sha3_512,Digest};
use rand::{Rng,thread_rng};
use rand::distributions::Alphanumeric;

use crate::database::user_db::*;
use crate::models::user::User;
use crate::controller::{get_id_from_request, get_user_by_claims, check_admin};
use crate::security::{get_claims_with_handler, SALT};

#[derive(Serialize,Deserialize)]
struct UserInsertData {
    password: String,
    email: String
}

pub fn r_get_all(request: &mut Request) -> IronResult<Response> {
    trace!("user -> r_get_all");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(claims) => {
            let id = get_user_by_claims(&claims);
            // only admin can access this request
            match check_admin(id) {
                Ok(_user) => {
                    h_get_all()
                },
                Err(e) => e
            }
        },
        Err(e) => e
    }
}

fn h_get_all() -> IronResult<Response> {
    let users = find_all();
    build_response_from_users(&users)
}

pub fn r_get_one(request: &mut Request) -> IronResult<Response> {
    trace!("user -> r_get_one");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(_claims) => {
            // only admin can access this request or the user limited to his data
            let id = get_id_from_request(request);
            match id {
                Ok(id) => h_get_one(id),
                Err(e) => e
            }
        },
        Err(e) => e
    }
}

fn h_get_one(id: i64) -> IronResult<Response> {
    let user = find_one(id);
    if user.is_ok() {
        let user = user.as_ref().ok().unwrap();
        if user.is_some() {
            let user = user.as_ref().unwrap();
            if user.id != id {
                warn!("The user tries to access protected data: {}", id);
                return Ok(Response::with(status::Unauthorized))
            }
        }
    }
    build_response_from_user(&user)
}

pub fn r_get_enabled(request: &mut Request) -> IronResult<Response> {
    trace!("user -> r_get_enabled");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(claims) => {
            let id = get_user_by_claims(&claims);
            // only admin can access this request
            match check_admin(id) {
                Ok(_user) => h_get_enabled(),
                Err(e) => e
            }
        },
        Err(e) => e
    }
}

fn h_get_enabled() -> IronResult<Response> {
    let users = find_enabled();
    build_response_from_users(&users)
}

pub fn r_disable(request: &mut Request) -> IronResult<Response> {
    trace!("user -> r_disable");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(claims) => {
            let id = get_user_by_claims(&claims);
            // only admin can access this request
            match check_admin(id) {
                Ok(_user) => {
                    let id = get_id_from_request(request);
                    match id {
                        Ok(id) => h_disable(id),
                        Err(e) => e
                    }
                },
                Err(e) => e
            }
        },
        Err(e) => e
    }
}

fn h_disable(id: i64) -> IronResult<Response> {
    let user = find_one(id);
    match user {
        Ok(user) => {
            match user {
                Some(mut user) => {
                    user.enable = false;
                    match update(&user) {
                        Ok(_n) => Ok(Response::with(status::NoContent)),
                        Err(_e) => {
                            error!("Unable to disable user!");
                            Ok(Response::with(status::InternalServerError))
                        }
                    }
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

pub fn r_insert(request: &mut Request) -> IronResult<Response> {
    trace!("user -> r_insert");
    h_insert(request)
}

fn h_insert(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let data: Result<UserInsertData, serde_json::error::Error> = serde_json::from_str(&payload);
    match data {
        Ok(data) => {
            let salt: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(SALT)
                .collect();
            let mut hasher = Sha3_512::new();
            hasher.input(data.password + &salt);
            let pwd = hex::encode(hasher.result());
            let user = User {
                id: 0,
                password: pwd,
                salt,
                email: data.email,
                last_login: None,
                enable: true,
                superuser: false
            };
            match insert(&user) {
                Ok(_n) => Ok(Response::with(status::NoContent)),
                Err(_e) => {
                    error!("Server Error!");
                    Ok(Response::with(status::InternalServerError))
                }
            }
        },
        Err(e) => {
            error!("{}", e);
            Ok(Response::with(status::BadRequest))
        }
    }
}

pub fn r_delete(request: &mut Request) -> IronResult<Response> {
    trace!("user -> r_delete");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(claims) => {
            let id = get_user_by_claims(&claims);
            // only admin can access this request
            match check_admin(id) {
                Ok(_user) => {
                    let id = get_id_from_request(request);
                    match id {
                        Ok(id) => h_delete(id),
                        Err(e) => e
                    }
                },
                Err(e) => e
            }
        },
        Err(e) => e
    }
}

fn h_delete(id: i64) -> IronResult<Response> {
    let user = find_one(id);
    match user {
        Ok(user) => {
            match user {
                Some(_user) => {
                    match delete(id) {
                        Ok(_n) => Ok(Response::with(status::NoContent)),
                        Err(_e) => {
                            error!("Server Error!");
                            Ok(Response::with(status::InternalServerError))
                        }
                    }
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

fn build_response_from_users(users: &Result<Option<std::vec::Vec<User>>, ()>) -> IronResult<Response> {
    match users {
        Ok(users) => {
            match users {
                Some(users) => {
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("application/json".parse::<Mime>().unwrap());
                    response.set_mut(serde_json::to_string(&users).unwrap());
                    Ok(response)
                },
                None => Ok(Response::with(status::NoContent))
            }
        },
        Err(_) => {
            error!("Server Error!");
            Ok(Response::with(status::InternalServerError))
        }
    }
}

fn build_response_from_user(user: &Result<Option<User>, ()>) -> IronResult<Response> {
    match user {
        Ok(user) => {
            match user {
                Some(user) => {
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("application/json".parse::<Mime>().unwrap());
                    response.set_mut(serde_json::to_string(&user).unwrap());
                    Ok(response)
                },
                None => Ok(Response::with(status::NoContent))
            }
        },
        Err(_e) => {
            error!("Server Error!");
            Ok(Response::with(status::InternalServerError))
        }
    }
}