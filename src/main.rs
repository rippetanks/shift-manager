
extern crate postgres;

#[macro_use]
extern crate log;
extern crate simplelog;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub mod models;
pub mod controller;
pub mod security;
pub mod database;

mod shift_log;

fn main() {
    // init log
    shift_log::init();
    info!("Here we go again!");
    // param
    let param = load_param();
    let jwt = param.get("JWT").unwrap().clone();
    let host = param.get("HOST").unwrap().clone();
    let pg_user = param.get("PG_USER").unwrap().clone();
    let pg_pwd = param.get("PG_PWD").unwrap().clone();
    let pg_host = param.get("PG_HOST").unwrap().clone();
    // init security
    security::jwt::init(jwt);
    info!("Security ready!");
    // init pool
    database::db::init(&pg_user, &pg_pwd, &pg_host);
    info!("Database pool ready!");
    // init REST Web Service
    controller::init(&host);
}

fn load_param() -> HashMap<String, String> {
    let f = if cfg!(windows) {
        File::open("C:\\shift_manager\\config.conf").unwrap()
    } else {
        File::open("/etc/shift_manager/config.conf").unwrap()
    };
    let mut map: HashMap<String, String> = HashMap::new();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let s: Vec<&str> = l.split("=").collect();
        map.insert(s[0].to_owned(), s[1].to_owned());
    }
    map
}
