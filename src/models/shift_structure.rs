
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct ShiftStructure {
    pub id: i64,
    pub id_user: i64,
    pub day: chrono::NaiveDate
}