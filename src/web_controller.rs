
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use rocket::response::NamedFile;
use rocket::response::status::NotFound;
use rocket::State;

use crate::controller::Extras;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/login")]
fn login() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("login", &context)
}

/*
#[get("/settings")]
fn settings() -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    Template::render("settings", &context)
}
*/

#[get("/settings")]
fn settings(extras: State<Extras>) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new(&extras.template_dir).join("settings.html");
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

#[get("/static/<file..>")]
fn serve_static(file: PathBuf, extras: State<Extras>) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new(&extras.static_dir).join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/", routes![index, settings, login, serve_static])
}
