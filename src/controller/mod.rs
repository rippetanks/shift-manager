
extern crate router;
extern crate iron;
extern crate iron_cors;

use router::Router;
use iron_cors::CorsMiddleware;
use iron::prelude::Iron;
use iron::status;
use iron::{Request, Chain, IronResult, Response};
use std::collections::HashSet;

use crate::security::jwt::Claims;
use crate::database::user_db;
use crate::models::user::User;

mod rest_user;
mod rest_auth;
mod rest_shift_structure;
mod rest_shift_expansion;

pub fn init() {
    let mut router = Router::new();

    let allowed_hosts = ["http://localhost", "https://rippetanks.ddns.net"].iter()
        .map(ToString::to_string).collect::<HashSet<_>>();
    info!("Allowed origin hosts: {:?}", allowed_hosts);
    let cors_middleware = CorsMiddleware::with_whitelist(allowed_hosts);

    // user
    router.get("/user", rest_user::r_get_all, "user_all");
    router.get("/user/:id", rest_user::r_get_one, "user_one");
    router.get("/user/enabled", rest_user::r_get_enabled, "user_enabled");
    router.post("/user/disable/:id", rest_user::r_disable, "user_disable");
    router.put("/user", rest_user::r_insert, "user_insert");
    router.delete("/user/:id", rest_user::r_delete, "user_delete");

    // auth
    router.post("/login", rest_auth::login, "login");
    router.post("/logout", rest_auth::logout, "logout");
    router.post("/keepalive", rest_auth::keepalive, "keepalive");

    // shift structure
    router.get("/structure", rest_shift_structure::r_get_by_user, "shift_s_by_user");
    router.get("/structure/:id", rest_shift_structure::r_get_one, "shift_s_one");
    router.put("/structure", rest_shift_structure::r_insert, "shift_s_insert");
    router.post("/structure", rest_shift_structure::r_update, "shift_s_update");
    router.delete("/structure/:id", rest_shift_structure::r_delete, "shift_s_delete");

    // shift expansion
    router.get("/expansion/structure/:id", rest_shift_expansion::r_get_by_structure, "shift_e_by_structure");
    router.get("/expansion/:id", rest_shift_expansion::r_get_one, "shift_e_one");
    router.put("/expansion/:id", rest_shift_expansion::r_insert, "shift_e_insert");
    router.post("/expansion/:id", rest_shift_expansion::r_update, "shift_e_update");
    router.delete("/expansion/:id", rest_shift_expansion::r_delete, "shift_e_delete");

    let mut chain = Chain::new(router);
    chain.link_around(cors_middleware);

    info!("Serving on http://localhost:3000...");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn get_id_from_request(_request: &mut Request) -> Result<i64, IronResult<Response>> {
    let params = _request.extensions.get::<Router>().unwrap();
    let id_str = &params.find("id");
    match id_str {
        Some(id_str) => Ok(id_str.parse().unwrap()),
        None => {
            error!("ID param not found!");
            Err(Ok(Response::with(status::BadRequest)))
        }
    }
}

fn get_user_by_claims(claims: &Claims) -> i64 {
    claims.sub.parse::<i64>().unwrap()
}

fn check_admin(id: i64) -> Result<User, IronResult<Response>> {
    let user = user_db::find_one(id);
    match user {
        Ok(user) => {
            match user {
                Some(user) => {
                    if user.superuser {
                        Ok(user)
                    } else {
                        error!("User is not a superuser!");
                        Err(Ok(Response::with(status::Unauthorized)))
                    }
                },
                None => {
                    error!("User not found!");
                    Err(Ok(Response::with(status::NotFound)))
                }
            }
        },
        Err(_) => {
            error!("Server error!");
            Err(Ok(Response::with(status::InternalServerError)))
        }
    }
}