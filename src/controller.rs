
use rocket_contrib::templates::Template;
use rocket::fairing::AdHoc;
use rocket::error::LaunchError;

use crate::database::ShiftManagerDB;
use crate::web_controller;
use crate::user;
use crate::shift_structure;
use crate::shift_expansion;

#[derive(Debug)]
pub struct Extras {
    pub jwt_key: String,
    pub jwt_exp: u64,
    pub template_dir: String,
    pub static_dir: String
}

pub fn init() -> LaunchError {
    let mut rocket = rocket::ignite()
        .attach(ShiftManagerDB::fairing())
        .attach(Template::fairing())
        .attach(fairing_extra());

    rocket = web_controller::mount(rocket);

    rocket = user::mount(rocket);
    rocket = shift_structure::mount(rocket);
    rocket = shift_expansion::mount(rocket);

    rocket.launch()
}

fn fairing_extra() -> rocket::fairing::AdHoc {
    AdHoc::on_attach("Extras Fairing", |rocket| {
        let config = rocket.config();
        let jwt_key = config.get_string("jwt_key").unwrap().to_string();
        let jwt_exp = config.get_int("jwt_exp").unwrap() as u64;
        let t_dir = config.get_string("template_dir").unwrap().to_string();
        let s_dir = config.get_string("static_dir").unwrap().to_string();
        Ok(rocket.manage(Extras {
            jwt_key,
            jwt_exp,
            template_dir: t_dir,
            static_dir: s_dir
        }))
    })
}
