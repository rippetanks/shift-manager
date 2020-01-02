
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::shift_structure::model::ShiftStructure;
use crate::shift_expansion::model::ShiftExpansion;

pub trait BaseModel<T> {
    fn unpack(result: Result<Vec<T>, Error>) -> Result<Json<Vec<T>>, Custom<String>> {
        match result {
            Ok(result) => {
                if result.len() != 0 {
                    Ok(Json(result))
                } else {
                    debug!("Unpack no content!");
                    Err(Custom(Status::NoContent, String::new()))
                }
            },
            Err(e) => {
                error!("An error occurred during unpack: {}", e);
                Err(Custom(Status::InternalServerError, e.to_string()))
            }
        }
    }
}

impl BaseModel<ShiftStructure> for ShiftStructure { }
impl BaseModel<ShiftExpansion> for ShiftExpansion { }
