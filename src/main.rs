#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate rocket;
extern crate serde;
use rocket::State;
use crate::lib::app::App;
use crate::lib::scope::Scope;
use serde::Deserialize;
use rocket_contrib::json::Json;

mod lib;

#[derive(Deserialize)]
struct Req {
    exprs: Vec<String>,
}

#[get("/<a>/<b>/<c>/<d>/<e>/<f>")]
fn req_get(app: State<App>, a: bool, b: bool, c: bool, d: f64, e: i64, f: i64) -> String {
    let scope = Scope::new(a, b, c, d, e, f);
    match app.run(scope) {
        Ok(f) => format!("Ok: {}", f),
        Err(e) => e.to_string()
    }
}

#[post("/<a>/<b>/<c>/<d>/<e>/<f>", data = "<exprs>")]
fn req_post(app: State<App>, a: bool, b: bool, c: bool, d: f64, e: i64, f: i64, exprs: Json<Req>) -> String {
    let app: Result<App, anyhow::Error> = exprs.exprs.iter().fold(Ok(app.clone()), |app, cur| {
        match app {
            Ok(a) => a.add(cur),
            Err(e) => Err(e)
        }
    });

    if app.is_err() {
        return app.unwrap_err().to_string();
    }

    let scope = Scope::new(a, b, c, d, e, f);
    match app.unwrap().run(scope) {
        Ok(f) => format!("Ok: {}", f),
        Err(e) => e.to_string()
    }
}

fn create(app: App) -> rocket::Rocket {
    rocket::ignite().manage(app).mount("/", routes![req_get, req_post])
}

fn main() {
    let app = App::default();
    create(app).launch();
}

#[cfg(test)]
mod test {
    use crate::lib::app::App;
    use crate::create;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn test_rocket() {
        let app = App::default();
        let r = create(app);
        let client = Client::new(r).expect("valid rocket instance");
        let req = client.get("/true/true/true/1.0/52/1");
        let mut response = req.dispatch();
        assert_eq!(response.body_string(), Some("Ok: 3".to_string()));

        // override rule
        let req = client.post("/true/true/false/1.0/52/1");
        let req = req.body("{\"exprs\": [\"A && B && !C => H = P\"]}");
        let mut response = req.dispatch();
        assert_eq!(response.body_string(), Some("Ok: 3".to_string()));

        // add custom rule
        let req = client.post("/false/false/false/1.0/52/1");
        let req = req.body("{\"exprs\": [\"!A && !B && !C => H = P\"]}");
        let mut response = req.dispatch();
        assert_eq!(response.body_string(), Some("Ok: 3".to_string()));

        // add trash
        let req = client.post("/false/false/false/1.0/52/1");
        let req = req.body("{\"exprs\": [\"!A && !B && !\"]}");
        let mut response = req.dispatch();
        assert_eq!(response.body_string(), Some("invalid expression".to_string()));
    }
}


