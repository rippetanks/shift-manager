
extern crate postgres;

use crate::database::db::*;
use crate::models::user::*;
use crate::database::handle_pg_update;
use crate::database::handle_pg_result;

pub fn find_all() -> Result<Option<Vec<User>>, ()> {
    trace!("user_db -> find_all()");
    let conn = get_connection();
    let result = conn.query("SELECT * FROM \"User\"", &[]);
    let rows = handle_pg_result(result)?;
    Ok(convert_list_handler(&rows))
}

pub fn find_enabled() -> Result<Option<Vec<User>>, ()> {
    trace!("user_db -> find_enabled()");
    let conn = get_connection();
    let result = conn.query("SELECT * FROM \"User\" WHERE enable = 1", &[]);
    let rows = handle_pg_result(result)?;
    Ok(convert_list_handler(&rows))
}

pub fn find_one(id: i64) -> Result<Option<User>, ()> {
    trace!("user_db -> find_one({})", id);
    let conn = get_connection();
    let result = conn.query("SELECT * FROM \"User\" WHERE id = $1", &[&id]);
    let rows = handle_pg_result(result)?;
    Ok(convert_handler(&rows))
}

pub fn find_by_email(email: &String) -> Result<Option<User>, ()> {
    trace!("user_db -> find_by_email({})", email);
    let conn = get_connection();
    let result = conn.query("SELECT * FROM \"User\" WHERE email = $1", &[email]);
    let rows = handle_pg_result(result)?;
    Ok(convert_handler(&rows))
}

pub fn insert(user: &User) -> Result<u64, ()> {
    trace!("user_db -> insert({})", user.email);
    let conn = get_connection();
    let result = conn.execute("INSERT INTO \"User\" (password, salt, email, enable) VALUES ($1, $2, $3, $4)",
        &[&user.password, &user.salt, &user.email, &user.enable]);
    handle_pg_update(result)
}

pub fn update(user: &User) -> Result<u64, ()> {
    trace!("user_db -> update({})", user.id);
    let conn = get_connection();
    let result = conn.execute("UPDATE \"User\" SET password = $1, salt = $2, email = $3, enable = $4 WHERE id = $5",
        &[&user.password, &user.salt, &user.email, &user.enable, &user.id]);
    handle_pg_update(result)
}

pub fn delete(id: i64) -> Result<u64, ()> {
    trace!("user_db -> delete({})", id);
    let conn = get_connection();
    let result = conn.execute("DELETE FROM \"User\" WHERE id = $1", &[&id]);
    handle_pg_update(result)
}

fn convert_list_handler(rows: &postgres::rows::Rows) -> Option<Vec<User>> {
    if rows.len() != 0 {
        Some(convert_list(&rows))
    } else {
        debug!("user_db -> no rows!");
        None
    }
}

fn convert_handler(rows : &postgres::rows::Rows) -> Option<User> {
    if rows.len() != 0 {
        Some(convert(&rows.get(0)))
    } else {
        debug!("user_db -> no row!");
        None
    }
}

fn convert_list(rows : &postgres::rows::Rows) -> Vec<User> {
    let mut v = vec![];
    for r in rows {
        let user = convert(&r);
        v.push(user);
    }
    v
}

fn convert(r : &postgres::rows::Row) -> User {
    let pwd: String = r.get(1);
    let salt: String = r.get(5);
    User {
        id: r.get(0),
        password: pwd.trim().to_string(),
        salt: salt.trim().to_string(),
        email: r.get(2),
        last_login: r.get(3),
        enable: r.get(4),
        superuser: r.get(6)
    }
}