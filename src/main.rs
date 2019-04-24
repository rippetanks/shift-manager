
extern crate postgres;

#[macro_use]
extern crate log;
extern crate simplelog;

pub mod models;
pub mod controller;
pub mod security;
pub mod database;

mod shift_log;

fn main() {
    // init log
    shift_log::init();
    info!("Here we go again!");
    // init pool
    database::db::init();
    info!("Database pool ready!");
    // init REST Web Service
    controller::init();
}
