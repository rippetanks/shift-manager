
extern crate postgres;

#[macro_use]
extern crate log;
extern crate simplelog;

use std::env;
use std::process::exit;

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
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        error!("Missing params -> require JWT secret, postgres username, postgres password");
        exit(-1);
    }
    // init security
    security::jwt::init(args.get(1).unwrap().to_owned());
    info!("Security ready!");
    // init pool
    database::db::init(args.get(2).unwrap(), args.get(3).unwrap(), args.get(4).unwrap());
    info!("Database pool ready!");
    // init REST Web Service
    controller::init();
}
