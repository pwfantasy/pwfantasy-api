extern crate rocket;

use rocket::{State};
use rocket_contrib::{JSON, Value};
use rocket::{Request, Route, Data, Catcher, Error};
use rocket::http::Status;

use mysql;
use talent;

#[put("/<slug>", format = "application/json", data = "<superstar>")]
pub fn create(pool: State<mysql::Pool>, slug: String, superstar: JSON<talent::Talent>) -> JSON<Value> {
	// need to require admin privledges
    JSON(json!({ "status": "ok" }))
}

#[get("/<slug>", format = "application/json")]
#[allow(unmounted_route)]
pub fn retrieve(pool: State<mysql::Pool>, slug: String) -> Option<JSON<Value>> {

	let talents: Vec<talent::Talent> = talent::retrieve_by_slug(pool, slug);

    Some(JSON(json!(talents)))

}
