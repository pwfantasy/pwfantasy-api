
use rocket::{State};
use mysql;

#[derive(Debug, Serialize, Deserialize)]
pub struct Talent {
    id: i32,
    name: String,
    short_name: Option<String>,
    slug: Option<String>,
    tier: i32,
    active: Option<i32>,
    faction: Option<i32>,
    championship: Option<i32>,
    promotion: i32
}

pub fn retrieve_by_slug(pool: State<mysql::Pool>, slug: String) -> Vec<Talent> {
    let params = params!{
        "slug" => slug
    };

    // Let's select payments from database
    let talents: Vec<Talent> =
    pool.prep_exec("SELECT superstarId, superstarName, tierId, promotionId, championshipId FROM superstars WHERE superstarSlug = :slug", params)
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (superstarId, superstarName, tierId, promotionId, championshipId) = mysql::from_row(row);

            Talent {
                id: superstarId,
                name: superstarName,
                tier: tierId,
                promotion: promotionId,
                active: None,
                faction: None,
                championship: championshipId,
                short_name: None,
                slug: None,
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`

    talents
}