extern crate rocket;

use rocket_contrib::{JSON, Value};
use rocket::{State, Request, Route, Data, Catcher, Error};
use rocket::http::Status;

use mysql;
use talent;

#[put("/<slug>", format = "application/json", data = "<superstar>")]
#[allow(unused_variables)]
pub fn create(pool: State<mysql::Pool>, slug: String, superstar: JSON<talent::Talent>) -> JSON<Value> {
    talent::upsert_superstar(pool, superstar.into_inner());

    // need to require admin privledges
    JSON(json!({ "status": "ok" }))
}

#[get("/<slug>", format = "application/json")]
#[allow(unmounted_route)]
pub fn retrieve(pool: State<mysql::Pool>, slug: String) -> Option<JSON<Value>> {
    let talent: Option<talent::Talent> = talent::retrieve_by_slug(pool, slug);

    if talent.is_none() {
        return None;
    }
    
    Some(JSON(json!(talent)))
}

#[get("/search/<term>", format = "application/json")]
pub fn search(pool: State<mysql::Pool>, term: String) -> Option<JSON<Value>> {
    let results: Vec<talent::Talent> = talent::search_by_term(pool, term);

    if results.len() < 1 {
       return None;
    }

    Some(JSON(json!(results)))
}
