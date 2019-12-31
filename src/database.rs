
use rocket_contrib::databases::diesel;

#[database("db")]
pub struct ShiftManagerDB(diesel::PgConnection);
