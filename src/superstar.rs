extern crate rocket;

use rocket::{State};
use rocket_contrib::{JSON, Value};
use rocket::{Request, Route, Data, Catcher, Error};
use rocket::http::Status;

use mysql;
use talent;

#[put("/<slug>", format = "application/json", data = "<superstar>")]
#[allow(unused_variables)]
pub fn create(pool: State<mysql::Pool>, slug: String, superstar: JSON<talent::Talent>) -> JSON<Value> {
    // need to require admin privledges
    JSON(json!({ "status": "ok" }))
}

#[get("/<slug>", format = "application/json")]
#[allow(unmounted_route)]
pub fn retrieve(pool: State<mysql::Pool>, slug: String) -> Option<JSON<Value>> {
    let talent: Option<talent::Talent> = talent::retrieve_by_slug(pool, slug);

    if talent.is_some() {
        Some(JSON(json!(talent)))
    } else {
        None
    }
}

#[get("/search/<term>", format = "application/json")]
pub fn search(pool: State<mysql::Pool>, term: String) -> Option<JSON<Value>> {
    let results: Vec<talent::Talent> = talent::search_by_term(pool, term);

    if results.len() > 0 {
        Some(JSON(json!(results)))
    } else {
        None
    }
}
