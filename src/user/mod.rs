
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::State;
use diesel::result::Error;

use crate::user::model::{User, UserForm};
use crate::controller::Extras;
use crate::database::ShiftManagerDB;

pub mod model;
pub mod auth;

#[derive(Debug,Deserialize)]
struct AuthJSON<'a> {
    email: &'a str,
    password: &'a str
}

#[put("/", data = "<json>", format = "application/json")]
fn create(conn: ShiftManagerDB, json: Json<AuthJSON>) -> Result<Status, Status> {
    debug!("CREATE_USER_REQUEST");
    let auth = auth::create_auth(json.password).unwrap();
    let user = UserForm {
        pwd: auth.stored_key.as_str(),
        email: json.email,
        last_login: None,
        enable: true,
        salt: auth.salt.as_str(),
        superuser: false
    };
    match User::create(&user, &conn) {
        Ok(u) => {
            info!("user create successfully for user {}!", u.id);
            Ok(Status::NoContent)
        },
        Err(e) => {
            error!("Can not create user caused by {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/login", data = "<json>", format = "application/json")]
fn login(conn: ShiftManagerDB, json: Json<AuthJSON>, extra: State<Extras>) -> Result<String, Status> {
    debug!("LOGIN_REQUEST");
    let user = User::read_by_email(json.email, &conn);
    match user {
        Ok(user) if auth::login(&user, json.password, &conn) => {
            finalize_login(&user, &json, &extra)
        },
        Ok(_) => {
            warn!("Wrong credential! Cant not login the user: {}", json.email);
            Err(Status::Unauthorized)
        },
        Err(e) if e.eq(&Error::NotFound) => {
            error!("Can not login the user {}: {}", json.email, e);
            Err(Status::NotFound)
        },
        Err(e) => {
            error!("Can not login the user {}: {}", json.email, e);
            Err(Status::InternalServerError)
        }
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/user", routes![create, login])
}

// #################################################################################################

fn finalize_login(user: &User, json: &Json<AuthJSON>, extra: &State<Extras>) -> Result<String, Status> {
    let token = auth::create_token(&user, &extra);
    match token {
        Ok(t) => {
            info!("The user {} has just logged in!", user.id);
            Ok(json!({"token": t}).to_string())
        },
        Err(s) => {
            error!("Can not login the user: {}", json.email);
            Err(s)
        }
    }
}