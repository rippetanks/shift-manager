
extern crate mime;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use std::io::Read;
use serde::{Deserialize, Serialize};

use crate::database::shift_structure_db::*;
use crate::models::shift_structure::ShiftStructure;
use crate::security::*;
use crate::controller::{get_id_from_request, get_user_by_claims};

#[derive(Serialize,Deserialize)]
struct ShiftInsertData {
    day: chrono::NaiveDate
}

#[derive(Serialize,Deserialize)]
struct ShiftUpdateData {
    id: i64,
    id_user: i64,
    day: chrono::NaiveDate
}

pub fn r_get_by_user(request: &mut Request) -> IronResult<Response> {
    trace!("shift_structure -> r_get_by_user");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(claims) => {
            let id = get_user_by_claims(&claims);
            h_get_by_user(id)
        },
        Err(e) => e
    }
}

fn h_get_by_user(id: i64) -> IronResult<Response> {
    let structure = find_by_user(id);
    build_response_from_structure(structure)
}

pub fn r_get_one(request: &mut Request) -> IronResult<Response> {
    trace!("shift_structure -> r_get_one");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(_claims) => {
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
    let structure = find_one(id);
    build_response_from_structure(structure)
}

pub fn r_insert(request: &mut Request) -> IronResult<Response> {
    trace!("shift_structure -> r_insert");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(claims) => {
            let id = get_user_by_claims(&claims);
            h_insert(id, request)
        },
        Err(e) => e
    }
}

fn h_insert(id_user: i64, request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let data: Result<ShiftInsertData, serde_json::error::Error> = serde_json::from_str(&payload);
    match data {
        Ok(data) => {
            let shift_structure = ShiftStructure {
                id: 0,
                id_user,
                day: data.day
            };
            match insert(&shift_structure) {
                Ok(_n) => Ok(Response::with(status::NoContent)),
                Err(_) => {
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

pub fn r_update(request: &mut Request) -> IronResult<Response> {
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(_claims) => {
            h_update(request)
        },
        Err(e) => e
    }
}

fn h_update(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let data: Result<ShiftUpdateData, serde_json::error::Error> = serde_json::from_str(&payload);
    match data {
        Ok(data) => {
            let shift_structure = ShiftStructure {
                id: data.id,
                id_user: data.id_user,
                day: data.day
            };
            match update(&shift_structure) {
                Ok(_n) => Ok(Response::with(status::NoContent)),
                Err(_e) => Ok(Response::with(status::InternalServerError))
            }
        },
        Err(_e) => {
            // TODO log
            Ok(Response::with(status::BadRequest))
        }
    }
}

pub fn r_delete(request: &mut Request) -> IronResult<Response> {
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(_claims) => {
            let id = get_id_from_request(request);
            match id {
                Ok(id) => h_delete(id),
                Err(_e) => {
                    // TODO log
                    Ok(Response::with(status::BadRequest))
                }
            }
        },
        Err(e) => e
    }
}

fn h_delete(id: i64) -> IronResult<Response> {
    match delete(id) {
        Ok(_n) => {
            Ok(Response::with(status::NoContent))
        },
        Err(_) => {
            Ok(Response::with(status::InternalServerError))
        }
    }
}

fn build_response_from_structure(structure: Result<Option<ShiftStructure>, ()>) -> IronResult<Response> {
    match structure {
        Ok(structure) => {
            match structure {
                Some(structure) => {
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("application/json".parse::<Mime>().unwrap());
                    response.set_mut(serde_json::to_string(&structure).unwrap());
                    Ok(response)
                },
                None => Ok(Response::with(status::NoContent))
            }
        },
        Err(_e) => Ok(Response::with(status::InternalServerError))
    }
}