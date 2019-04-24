
extern crate postgres;

use crate::database::db::*;
use crate::models::shift_expansion::*;
use crate::database::{handle_pg_result, handle_pg_update};

pub fn find_all_by_structure(id: i64) -> Result<Option<Vec<ShiftExpansion>>, ()> {
    trace!("shift_expansion_db -> find_all_by_structure({})", id);
    let conn = get_connection();
    let result = conn.query("SELECT * FROM shift_expansion WHERE id_structure = $1 ORDER BY prog ASC",
                            &[&id]);
    let rows = handle_pg_result(result)?;
    Ok(convert_list_handler(&rows))
}

pub fn find_one(id: i64) -> Result<Option<ShiftExpansion>, ()> {
    trace!("shift_expansion_db -> find_one({})", id);
    let conn = get_connection();
    let result = conn.query("SELECT * FROM shift_expansion WHERE id = $1", &[&id]);
    let rows = handle_pg_result(result)?;
    Ok(convert_handler(&rows))
}

pub fn insert(shift: &ShiftExpansion) -> Result<u64, ()> {
    trace!("shift_expansion_db -> insert({}, {})", shift.id_structure, shift.prog);
    let conn = get_connection();
    let result =
        conn.execute("INSERT INTO shift_expansion (id_structure, morning, afternoon, night, rest, prog) VALUES ($1, $2, $3, $4, $5, $6)",
                 &[&shift.id_structure, &shift.morning, &shift.afternoon, &shift.night, &shift.rest, &shift.prog]);
    handle_pg_update(result)
}

pub fn update(shift: &ShiftExpansion) -> Result<u64, ()> {
    trace!("shift_expansion_db -> update({})", shift.id);
    let conn = get_connection();
    let result =
        conn.execute("UPDATE shift_expansion SET morning = $1, afternoon = $2, night = $3, rest = $4, prog = $5 WHERE id = $6",
                 &[&shift.morning, &shift.afternoon, &shift.night, &shift.rest, &shift.prog, &shift.id]);
    handle_pg_update(result)
}

pub fn delete(id: i64) -> Result<u64, ()> {
    trace!("shift_expansion_db -> delete({})", id);
    let conn = get_connection();
    let result = conn.execute("DELETE FROM shift_expansion WHERE id = $1", &[&id]);
    handle_pg_update(result)
}

fn convert_list_handler(rows: &postgres::rows::Rows) -> Option<Vec<ShiftExpansion>> {
    if rows.len() != 0 {
        debug!("shift_expansion_db -> no rows!");
        Some(convert_list(&rows))
    } else {
        None
    }
}

fn convert_handler(rows : &postgres::rows::Rows) -> Option<ShiftExpansion> {
    if rows.len() != 0 {
        debug!("shift_expansion_db -> no row!");
        Some(convert(&rows.get(0)))
    } else {
        None
    }
}

fn convert_list(rows : &postgres::rows::Rows) -> Vec<ShiftExpansion> {
    let mut v = vec![];
    for r in rows {
        let shift = convert(&r);
        v.push(shift);
    }
    v
}

fn convert(r : &postgres::rows::Row) -> ShiftExpansion {
    ShiftExpansion {
        id: r.get(0),
        id_structure: r.get(1),
        morning: r.get(2),
        afternoon: r.get(3),
        night: r.get(4),
        rest: r.get(5),
        prog: r.get(6)
    }
}