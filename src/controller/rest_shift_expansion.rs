
extern crate mime;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use std::io::Read;
use serde::{Deserialize, Serialize};

use crate::database::shift_expansion_db::*;
use crate::models::shift_expansion::ShiftExpansion;
use crate::security::*;
use crate::controller::get_id_from_request;

#[derive(Serialize,Deserialize)]
struct ShiftInsertData {
    morning: bool,
    afternoon: bool,
    night: bool,
    rest: bool,
    prog: i16
}

#[derive(Serialize,Deserialize)]
struct ShiftUpdateData {
    morning: bool,
    afternoon: bool,
    night: bool,
    rest: bool,
    prog: i16
}

pub fn r_get_by_structure(request: &mut Request) -> IronResult<Response> {
    trace!("shift_expansion -> r_get_by_structure");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(_claims) => {
            let id = get_id_from_request(request);
            match id {
                Ok(id) => h_get_by_structure(id),
                Err(e) => e
            }
        },
        Err(e) => e
    }
}

fn h_get_by_structure(id: i64) -> IronResult<Response> {
    let expansion = find_all_by_structure(id);
    build_response_from_expansions(expansion)
}

pub fn r_get_one(request: &mut Request) -> IronResult<Response> {
    trace!("shift_expansion -> r_get_one");
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
    let expansion = find_one(id);
    build_response_from_expansion(expansion)
}

pub fn r_insert(request: &mut Request) -> IronResult<Response> {
    trace!("shift_expansion -> r_insert");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(_claims) => {
            let id = get_id_from_request(request);
            match id {
                Ok(id) => h_insert(id, request),
                Err(e) => e
            }
        },
        Err(e) => e
    }
}

fn h_insert(id: i64, request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let data: Result<ShiftInsertData, serde_json::error::Error> = serde_json::from_str(&payload);
    match data {
        Ok(data) => {
            let shift_expansion = ShiftExpansion {
                id: 0,
                id_structure: id,
                morning: data.morning,
                afternoon: data.afternoon,
                night: data.night,
                rest: data.rest,
                prog: data.prog
            };
            match insert(&shift_expansion) {
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
    trace!("shift_expansion -> r_update");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(_claims) => {
            let id = get_id_from_request(request);
            match id {
                Ok(id) => h_update(id, request),
                Err(e) => e
            }
        },
        Err(e) => e
    }
}

fn h_update(id: i64, request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let data: Result<ShiftUpdateData, serde_json::error::Error> = serde_json::from_str(&payload);
    match data {
        Ok(data) => {
            let shift_expansion = ShiftExpansion {
                id,
                id_structure: 0,
                morning: data.morning,
                afternoon: data.afternoon,
                night: data.night,
                rest: data.rest,
                prog: data.prog
            };
            match update(&shift_expansion) {
                Ok(_n) => Ok(Response::with(status::NoContent)),
                Err(_) => {
                    error!("Server Error!");
                    Ok(Response::with(status::NoContent))
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
    trace!("shift_expansion -> r_delete");
    // get jwt claims
    match get_claims_with_handler(request) {
        Ok(_claims) => {
            let id = get_id_from_request(request);
            match id {
                Ok(id) => h_delete(id),
                Err(e) => e
            }
        },
        Err(e) => e
    }
}

fn h_delete(id: i64) -> IronResult<Response> {
    match delete(id) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(_) => {
            error!("Server Error!");
            Ok(Response::with(status::InternalServerError))
        }
    }
}

fn build_response_from_expansion(expansion: Result<Option<ShiftExpansion>, ()>) -> IronResult<Response> {
    match expansion {
        Ok(expansion) => {
            match expansion {
                Some(expansion) => {
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("application/json".parse::<Mime>().unwrap());
                    response.set_mut(serde_json::to_string(&expansion).unwrap());
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

fn build_response_from_expansions(expansions: Result<Option<std::vec::Vec<ShiftExpansion>>, ()>) -> IronResult<Response> {
    match expansions {
        Ok(expansions) => {
            match expansions {
                Some(expansions) => {
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("application/json".parse::<Mime>().unwrap());
                    response.set_mut(serde_json::to_string(&expansions).unwrap());
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