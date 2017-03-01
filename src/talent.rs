
use rocket::{State};
use mysql;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Talent {
    id: Option<i32>,
    name: String,
    slug: String,
    tier: i32,
    active: i32,
    faction: Option<i32>,
    championship: Option<i32>,
    show: Option<i32>,
    image: Option<String>,
    bio: Option<String>
}

pub fn retrieve_by_slug(pool: State<mysql::Pool>, slug: String) -> Option<Talent> {
    let params = params!{
        "slug" => slug
    };

    let talent: Option<Talent> =
        pool.prep_exec("SELECT id, `name`, slug, tier, active, faction, championship, `show`, image, bio FROM talent WHERE slug = :slug", params)
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
        pool.prep_exec("SELECT id, `name`, slug, tier, active, faction, championship, `show`, null as image, null as bio FROM talent WHERE name LIKE CONCAT('%', :term, '%') OR slug LIKE CONCAT('%', :term, '%')", params)
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let talent: Talent = row_to_talent(row);
                talent
            }).collect()
        }).unwrap();

    talents
}

pub fn upsert_superstar(pool: State<mysql::Pool>, talent: Talent) -> Result<String, String> {
    let params = params!{
        "id" => talent.id,
        "name" => talent.name,
        "slug" => &talent.slug,
        "tier" => talent.tier,
        "active" => talent.active,
        "faction" => talent.faction,
        "championship" => talent.championship,
        "show" => talent.show,
        "image" => talent.image,
        "bio" => talent.bio
    };
    
    let query = "
        INSERT INTO talent (id, `name`, slug, tier, active, faction, championship, `show`, image, bio) 
        VALUES (:id, :name, :slug, :tier, :active, :faction, :championship, :show, :image, :bio)
        ON DUPLICATE KEY UPDATE 
            `name` = VALUES(`name`),
            slug = VALUES(slug),
            tier = VALUES(tier),
            active = VALUES(active),
            faction = VALUES(faction),
            championship = VALUES(championship),
            `show` = VALUES(`show`),
            image = VALUES(image),
            bio = VALUES(bio)";

    let result = pool.prep_exec(query, params);

    match result.is_ok() {
        true => Ok(match result.unwrap().affected_rows() {
            0 => format!("{} record is the same", talent.slug),
            1 => format!("{} record created", talent.slug),
            2 => format!("{} record was updated", talent.slug),
            _ => format!("no clue what the hell happened")
        }),
        false => Err(format!("{}", result.unwrap_err().cause().unwrap()))
    }
    
    /*
        With ON DUPLICATE KEY UPDATE, the affected-rows value per row:
        1 if the row is inserted as a new row
        2 if an existing row is updated
        0 if an existing row is set to its current values.
    */
    // result.affected_rows();

    // let query = "
    //     INSERT INTO talent SET ?
    //     ON DUPLICATE KEY UPDATE 
    //         id = VALUES(id),
    //         `name` = VALUES(`name`),
    //         slug = VALUES(slug),
    //         tier = VALUES(tier),
    //         active = VALUES(active),
    //         faction = VALUES(faction),
    //         championship = VALUES(championship),
    //         `show` = VALUES(`show`),
    //         image = VALUES(image),
    //         bio = VALUES(bio)";

    // let mut stmt = pool.prepare(query).unwrap();
    // let result = stmt.execute(params).unwrap();

    // println!("{:?}", result);
}

fn row_to_talent(row: mysql::Row) -> Talent {
    let (id, name, slug, tier, active, faction, championship, show, image, bio) = mysql::from_row(row);

    Talent {
        id: id,
        name: name,
        slug: slug,
        tier: tier,
        active: active,
        faction: faction,
        championship: championship,
        show: show,
        image: image,
        bio: bio
    }
}