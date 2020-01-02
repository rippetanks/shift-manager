
use rocket::http::Status;
use diesel::QueryResult;

use crate::shift_structure::model::ShiftStructure;
use crate::shift_expansion::model::ShiftExpansion;

pub trait BaseController<T> {
    fn finalize_update_delete(result: QueryResult<usize>) -> Result<Status, Status> {
        match result {
            Ok(n) if n > 0 => Ok(Status::NoContent),
            Ok(_) => {
                warn!("object not found!");
                Err(Status::NotFound)
            },
            Err(e) => {
                error!("error on update/delete object: {}", e);
                Err(Status::InternalServerError)
            }
        }
    }
}

impl BaseController<ShiftStructure> for ShiftStructure { }
impl BaseController<ShiftExpansion> for ShiftExpansion { }
