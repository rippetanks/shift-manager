
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

#[macro_use] extern crate log;
extern crate log4rs;

mod base_model;
mod base_controller;
mod controller;
mod web_controller;
mod database;
mod schema;

mod user;
mod shift_structure;
mod shift_expansion;

fn main() {
    log4rs::init_file("log-config.yml", Default::default()).unwrap();

    let error = controller::init();
    error!("Launch failed! Error: {}", error);
}

