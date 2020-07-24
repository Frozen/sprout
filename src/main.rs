#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate anyhow;
#[macro_use] extern crate rocket;
use rocket::State;
use crate::lib::app::App;
use crate::lib::scope::Scope;
use rocket::response::status::BadRequest;

mod lib;

#[get("/<a>/<b>/<c>/<d>/<e>/<f>")]
fn hello(app: State<App>, a: bool, b: bool, c: bool, d: f64, e: i64, f: i64) -> String {
    let scope = Scope::new(a, b, c, d, e, f);
    match app.run(scope) {
        Ok(f) => format!("Ok: {}", f),
        Err(e) => e.to_string()
    }
}

fn main() {
    let app = App::new();
    let app = app.add("A && B && !C => H = M").unwrap();
    let app = app.add("A && B && C => H = P").unwrap();
    let app = app.add("!A && B && C => H = T").unwrap();
    let app = app.add("H = M => K = D + (D * E / 10)").unwrap();
    let app = app.add("H = P => K = D + (D * (E - F) / 25.5)").unwrap();
    let app = app.add("H = T => K = D - (D * F / 30)").unwrap();

    rocket::ignite().manage(app).mount("/", routes![hello]).launch();
}

