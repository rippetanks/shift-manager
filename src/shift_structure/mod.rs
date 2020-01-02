
use rocket_contrib::json::Json;
use rocket::http::Status;
use serde::Deserialize;
use diesel::result::Error;

use crate::database::ShiftManagerDB;
//use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::shift_structure::model::{ShiftStructure, ShiftStructureForm};
use crate::user::model::User;

pub mod model;

#[derive(Debug,Deserialize)]
struct StructureJSON {
    pub day: chrono::NaiveDate
}

#[put("/", data = "<json>", format = "application/json")]
fn create(conn: ShiftManagerDB, json: Json<StructureJSON>, user: User) -> Result<Json<ShiftStructure>, Status> {
    debug!("CREATE_SHIFT_STRUCTURE_REQUEST");
    let form = ShiftStructureForm {
        id_user: user.id,
        day: json.day
    };
    ShiftStructure::create(&form, &conn)
        .map(|result| {
            info!("shift structure create successfully: {}", result.id);
            Json(result)
        })
        .map_err(|e| {
            error!("Can not create shift structure: {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: ShiftManagerDB, id: i64, user: User) -> Result<Json<ShiftStructure>, Status> {
    debug!("READ_ONE_SHIFT_STRUCTURE_REQUEST");
    let ss = get_by_id(id, &conn)?;
    // a user can access his own shift structure
    check_property(&ss, &user)?;
    Ok(Json(ss))
}

#[get("/user")]
fn read_for_user(conn: ShiftManagerDB, user: User) -> Result<Json<ShiftStructure>, Status> {
    debug!("READ_SHIFT_STRUCTURE_FOR_USER_REQUEST");
    ShiftStructure::read_by_user(&user, &conn)
        .map(Json)
        .map_err(|e| {
            error!("Can not read shift structure: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

#[post("/<id>", data = "<json>", format = "application/json")]
fn update(conn: ShiftManagerDB, id: i64, json: Json<StructureJSON>, user: User) -> Result<Status, Status> {
    debug!("UPDATE_SHIFT_STRUCTURE_REQUEST");
    let form = ShiftStructureForm {
        id_user: user.id,
        day: json.day
    };
    let ss = get_by_id(id, &conn)?;
    // check if shift structure can be updated
    check_property(&ss, &user)?;
    let update = ShiftStructure::update(&ss, &form, &conn);
    ShiftStructure::finalize_update_delete(update)
}

#[delete("/<id>")]
fn delete(conn: ShiftManagerDB, id: i64, user: User) -> Result<Status, Status> {
    debug!("DELETE_SHIFT_STRUCTURE_REQUEST");
    let ss = get_by_id(id, &conn)?;
    // check if shift structure can be deleted
    check_property(&ss, &user)?;
    let delete = ShiftStructure::delete(&ss, &conn);
    ShiftStructure::finalize_update_delete(delete)
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/shift/structure", routes![create, read_one, read_for_user, update, delete])
}

///
///
pub fn get_and_check(id_structure: i64, user: &User, conn: &ShiftManagerDB) -> Result<ShiftStructure, Status> {
    let ss = get_by_id(id_structure, conn)?;
    check_property(&ss, user)?;
    Ok(ss)
}

// #################################################################################################

fn get_by_id(id: i64, conn: &ShiftManagerDB) -> Result<ShiftStructure, Status> {
    ShiftStructure::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read shift structure: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

fn check_property(ss: &ShiftStructure, user: &User) -> Result<(), Status> {
    if ss.id_user != user.id {
        warn!("The user attempts to access shift structure that does not belong to it!");
        Err(Status::Forbidden)
    } else {
        Ok(())
    }
}
