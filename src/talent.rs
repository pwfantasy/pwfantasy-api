
use rocket::{State};
use mysql;

#[derive(Debug, Serialize, Deserialize)]
pub struct Talent {
    id: i32,
    name: String,
    slug: String,
    tier: i32,
    active: i32,
    faction: Option<i32>,
    championship: Option<i32>,
    show: Option<i32>
}

pub fn retrieve_by_slug(pool: State<mysql::Pool>, slug: String) -> Option<Talent> {
    let params = params!{
        "slug" => slug
    };

    let talent: Option<Talent> =
        pool.prep_exec("SELECT id, `name`, slug, tier, active, faction, championship, `show` FROM talent WHERE slug = :slug", params)
        .map(|mut result| {
            let row = result.next();

            if row.is_none() {
                None
            } else {
                let row = row.unwrap().unwrap();
                let talent: Talent = row_to_talent(row);
                Some(talent)
            }
        }).unwrap();

    talent
}

pub fn search_by_term(pool: State<mysql::Pool>, term: String) -> Vec<Talent> {
    let params = params!{
        "term" => term
    };

    let talents: Vec<Talent> =
        pool.prep_exec("SELECT id, `name`, slug, tier, active, faction, championship, `show` FROM talent WHERE name LIKE CONCAT('%', :term, '%')", params)
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let talent: Talent = row_to_talent(row);
                talent
            }).collect()
        }).unwrap();

    talents
}

fn row_to_talent(row: mysql::Row) -> Talent {
    let (id, name, slug, tier, active, faction, championship, show) = mysql::from_row(row);

    Talent {
        id: id,
        name: name,
        slug: slug,
        tier: tier,
        active: active,
        faction: faction,
        championship: championship,
        show: show
    }
}