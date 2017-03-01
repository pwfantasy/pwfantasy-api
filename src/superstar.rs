extern crate rocket;

use rocket_contrib::{JSON, Value};
use rocket::{State};

use mysql;
use talent;

#[put("/<slug>", format = "application/json", data = "<superstar>")]
#[allow(unused_variables)]
pub fn upsert(pool: State<mysql::Pool>, slug: String, superstar: JSON<talent::Talent>) -> JSON<Value> {
    let upsert = talent::upsert_superstar(pool, superstar.into_inner());

    if upsert.is_ok() {
        return JSON(json!({
            "status": "ok",
            "message": upsert.unwrap()
        }))
    } else {
        return JSON(json!({
            "status": "error",
            "message": upsert.unwrap_err()
        }))
    }
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
