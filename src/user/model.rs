
use diesel;
use diesel::prelude::*;
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};

use crate::database::ShiftManagerDB;
use crate::schema::user;

#[table_name = "user"]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable)]
pub struct User {
    pub id: i64,
    pub pwd: String,
    pub email: String,
    pub last_login: Option<NaiveDateTime>,
    pub enable: bool,
    pub salt: String,
    pub superuser: bool
}

// only for insert and update
#[table_name = "user"]
#[derive(Debug,Insertable,AsChangeset)]
pub struct UserForm<'a> {
    pub pwd: &'a str,
    pub email: &'a str,
    pub last_login: Option<NaiveDateTime>,
    pub enable: bool,
    pub salt: &'a str,
    pub superuser: bool
}

impl User {
    pub fn create(user: &UserForm, conn: &ShiftManagerDB) -> QueryResult<User> {
        diesel::insert_into(user::table)
            .values(user)
            .get_result::<User>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &ShiftManagerDB) -> QueryResult<Vec<User>> {
        user::table.load::<User>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &ShiftManagerDB) -> QueryResult<User> {
        user::table.find(id).first::<User>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_email(email: &str, conn: &ShiftManagerDB) -> QueryResult<User> {
        user::table
            .filter(user::email.eq(email))
            .first(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(id: i64, user: &UserForm, conn: &ShiftManagerDB) -> QueryResult<usize> {
        diesel::update(user::table.find(id))
            .set(user)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update_last_login(id: i64, conn: &ShiftManagerDB) -> QueryResult<usize> {
        diesel::update(user::table.find(id))
            .set(user::last_login.eq(Utc::now().naive_utc()))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(id: i64, conn: &ShiftManagerDB) -> QueryResult<usize> {
        diesel::delete(user::table.find(id))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e ); e })
    }
    ///
    /// Not all info can be returned.
    pub fn mask(user: &mut User) {
        user.pwd = String::new();
        user.salt = String::new();
    }
}
