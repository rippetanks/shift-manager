
use diesel;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::database::ShiftManagerDB;
use crate::shift_structure::model::ShiftStructure;
use crate::schema::shift_expansion;

#[table_name = "shift_expansion"]
#[belongs_to(ShiftStructure, foreign_key = "id_structure")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Associations)]
pub struct ShiftExpansion {
    pub id: i64,
    pub id_structure: i64,
    pub morning: bool,
    pub afternoon: bool,
    pub night: bool,
    pub rest: bool,
    pub prog: i16
}

// only for insert and update
#[table_name = "shift_expansion"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct ShiftExpansionForm {
    pub id_structure: i64,
    pub morning: bool,
    pub afternoon: bool,
    pub night: bool,
    pub rest: bool,
    pub prog: i16
}

impl ShiftExpansion {
    pub fn create(se: &ShiftExpansionForm, conn: &ShiftManagerDB) -> QueryResult<ShiftExpansion> {
        diesel::insert_into(shift_expansion::table)
            .values(se)
            .get_result::<ShiftExpansion>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &ShiftManagerDB) -> QueryResult<Vec<ShiftExpansion>> {
        shift_expansion::table.load::<ShiftExpansion>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &ShiftManagerDB) -> QueryResult<ShiftExpansion> {
        shift_expansion::table.find(id).first::<ShiftExpansion>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_structure(ss: &ShiftStructure, conn: &ShiftManagerDB) -> QueryResult<Vec<ShiftExpansion>> {
        ShiftExpansion::belonging_to(ss)
            .load::<ShiftExpansion>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(se: &ShiftExpansion, form: &ShiftExpansionForm, conn: &ShiftManagerDB) -> QueryResult<usize> {
        diesel::update(shift_expansion::table.find(se.id))
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(se: &ShiftExpansion, conn: &ShiftManagerDB) -> QueryResult<usize> {
        diesel::delete(shift_expansion::table.find(se.id))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e ); e })
    }
}
