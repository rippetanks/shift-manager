
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use serde::Deserialize;
use diesel::result::Error;

use crate::database::ShiftManagerDB;
use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::shift_expansion::model::{ShiftExpansion, ShiftExpansionForm};
use crate::shift_structure;
use crate::user::model::User;

pub mod model;

#[derive(Debug,Deserialize)]
struct StructureJSON {
    pub morning: bool,
    pub afternoon: bool,
    pub night: bool,
    pub rest: bool,
    pub prog: i16
}

#[put("/<id>", data = "<json>", format = "application/json")]
fn create(conn: ShiftManagerDB, id: i64, json: Json<StructureJSON>, user: User) -> Result<Status, Status> {
    debug!("CREATE_SHIFT_EXPANSION_REQUEST");
    let ss = shift_structure::get_and_check(id, &user, &conn)?;
    let form = ShiftExpansionForm {
        id_structure: ss.id,
        morning: json.morning,
        afternoon: json.afternoon,
        night: json.night,
        rest: json.rest,
        prog: json.prog
    };
    ShiftExpansion::create(&form, &conn)
        .map(|se| {
            info!("shift expansion create successfully: {}", se.id);
            Status::NoContent
        })
        .map_err(|e| {
            error!("Can not create shift expansion: {}", e);
            Status::InternalServerError
        })
}


#[get("/<id>")]
fn read_one(conn: ShiftManagerDB, id: i64, user: User) -> Result<Json<ShiftExpansion>, Status> {
    debug!("READ_ONE_SHIFT_EXPANSION_REQUEST");
    let se = get_by_id(id, &conn)?;
    // a user can access his own shift expansion
    check_property(&se, &user, &conn)?;
    Ok(Json(se))
}

#[get("/structure/<id>")]
fn read_by_structure(conn: ShiftManagerDB, id: i64, user: User) -> Result<Json<Vec<ShiftExpansion>>, Custom<String>> {
    debug!("READ_SHIFT_EXPANSION_FOR_USER_REQUEST");
    let ss = shift_structure::get_and_check(id, &user, &conn)
        .map_err(|e| { Custom(e, String::new()) })?;
    let result = ShiftExpansion::read_by_structure(&ss, &conn);
    ShiftExpansion::unpack(result)
}

#[post("/<id>", data = "<json>", format = "application/json")]
fn update(conn: ShiftManagerDB, id: i64, json: Json<StructureJSON>, user: User) -> Result<Status, Status> {
    debug!("UPDATE_SHIFT_EXPANSION_REQUEST");
    let se = get_by_id(id, &conn)?;
    // check if shift expansion can be updated
    check_property(&se, &user, &conn)?;

    let form = ShiftExpansionForm {
        id_structure: se.id_structure,
        morning: json.morning,
        afternoon: json.afternoon,
        night: json.night,
        rest: json.rest,
        prog: json.prog
    };
    let update = ShiftExpansion::update(&se, &form, &conn);
    ShiftExpansion::finalize_update_delete(update)
}

#[delete("/<id>")]
fn delete(conn: ShiftManagerDB, id: i64, user: User) -> Result<Status, Status> {
    debug!("DELETE_SHIFT_EXPANSION_REQUEST");
    let se = get_by_id(id, &conn)?;
    // check if shift expansion can be deleted
    check_property(&se, &user, &conn)?;
    let delete = ShiftExpansion::delete(&se, &conn);
    ShiftExpansion::finalize_update_delete(delete)
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/shift/expansion", routes![create, read_one, read_by_structure, update, delete])
}

// #################################################################################################

fn get_by_id(id: i64, conn: &ShiftManagerDB) -> Result<ShiftExpansion, Status> {
    ShiftExpansion::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read shift expansion: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

fn check_property(se: &ShiftExpansion, user: &User, conn: &ShiftManagerDB) -> Result<(), Status> {
    shift_structure::get_and_check(se.id_structure, user, conn).map(|_| { })
}
