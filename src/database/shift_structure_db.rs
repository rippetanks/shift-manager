
extern crate postgres;

use crate::database::db::*;
use crate::models::shift_structure::*;
use crate::database::{handle_pg_result, handle_pg_update};

pub fn find_by_user(id: i64) -> Result<Option<ShiftStructure>, ()> {
    trace!("shift_structure_db -> find_by_user({})", id);
    let conn = get_connection();
    let result = conn.query("SELECT * FROM shift_structure WHERE id_user = $1", &[&id]);
    let rows = handle_pg_result(result)?;
    Ok(convert_handler(&rows))
}

pub fn find_one(id: i64) -> Result<Option<ShiftStructure>, ()> {
    trace!("shift_structure_db -> find_one({})", id);
    let conn = get_connection();
    let result = conn.query("SELECT * FROM shift_structure WHERE id = $1", &[&id]);
    let rows = handle_pg_result(result)?;
    Ok(convert_handler(&rows))
}

pub fn insert(shift: &ShiftStructure) -> Result<u64, ()> {
    trace!("shift_structure_db -> insert({}, {})", shift.id_user, shift.day);
    let conn = get_connection();
    let result = conn.execute("INSERT INTO shift_structure (id_user, day) VALUES ($1, $2)",
                 &[&shift.id_user, &shift.day]);
    handle_pg_update(result)
}

pub fn update(shift: &ShiftStructure) -> Result<u64, ()> {
    trace!("shift_structure_db -> update({})", shift.id);
    let conn = get_connection();
    let result = conn.execute("UPDATE shift_structure SET day = $1 WHERE id = $2",
                 &[&shift.day, &shift.id]);
    handle_pg_update(result)
}

pub fn delete(id: i64) -> Result<u64, ()> {
    trace!("shift_structure_db -> delete({})", id);
    let conn = get_connection();
    let result = conn.execute("DELETE FROM shift_structure WHERE id = $1", &[&id]);
    handle_pg_update(result)
}

fn convert_handler(row : &postgres::rows::Rows) -> Option<ShiftStructure> {
    if row.len() != 0 {
        Some(convert(&row.get(0)))
    } else {
        debug!("shift_structure_db -> no row!");
        None
    }
}

fn convert(r : &postgres::rows::Row) -> ShiftStructure {
    ShiftStructure {
        id: r.get(0),
        id_user: r.get(1),
        day: r.get(2)
    }
}