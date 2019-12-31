
use diesel;
use diesel::prelude::*;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

use crate::database::ShiftManagerDB;
use crate::schema::shift_structure;
use crate::user::model::User;

#[table_name = "shift_structure"]
#[belongs_to(User, foreign_key = "id_user")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Associations)]
pub struct ShiftStructure {
    pub id: i64,
    pub id_user: i64,
    pub day: NaiveDate
}

// only for insert and update
#[table_name = "shift_structure"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct ShiftStructureForm {
    pub id_user: i64,
    pub day: NaiveDate
}

impl ShiftStructure {
    pub fn create(ss: &ShiftStructureForm, conn: &ShiftManagerDB) -> QueryResult<ShiftStructure> {
        diesel::insert_into(shift_structure::table)
            .values(ss)
            .get_result::<ShiftStructure>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &ShiftManagerDB) -> QueryResult<Vec<ShiftStructure>> {
        shift_structure::table.load::<ShiftStructure>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &ShiftManagerDB) -> QueryResult<ShiftStructure> {
        shift_structure::table.find(id).first::<ShiftStructure>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_user(user: &User, conn: &ShiftManagerDB) -> QueryResult<ShiftStructure> {
        /*shift_structure::table
            .filter(shift_structure::id_user.eq(user.id))
            .first(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })*/
        ShiftStructure::belonging_to(user)
            .first(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(ss: &ShiftStructure, form: &ShiftStructureForm, conn: &ShiftManagerDB) -> QueryResult<usize> {
        diesel::update(shift_structure::table.find(ss.id))
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(ss: &ShiftStructure, conn: &ShiftManagerDB) -> QueryResult<usize> {
        diesel::delete(shift_structure::table.find(ss.id))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e ); e })
    }
}
