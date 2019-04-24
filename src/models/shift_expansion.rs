
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct ShiftExpansion {
    pub id: i64,
    pub id_structure: i64,
    pub morning: bool,
    pub afternoon: bool,
    pub night: bool,
    pub rest: bool,
    pub prog: i16
}