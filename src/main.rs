#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate mysql;

use rocket::{Rocket};
use rocket_contrib::{JSON, Value};

mod superstar;
mod talent;

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> Rocket {
    let mut opts = mysql::OptsBuilder::new();
    opts.db_name(Some("pwfantasy"));
    opts.prefer_socket(false); // only set to "false" when on windows
    opts.user(Some(env!("PWFDB_USER")));
    opts.pass(Some(env!("PWFDB_PASS")));

    let pool = mysql::Pool::new(opts).unwrap();

    rocket::ignite()
        .manage(pool)
        .mount("/superstar", routes![
            superstar::create,
            superstar::retrieve,
            superstar::search,
        ])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}