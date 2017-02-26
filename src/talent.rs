
use rocket::{State};
use mysql;
use mysql::Error::DriverError;

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

pub fn upsert_superstar(pool: State<mysql::Pool>, talent: Talent) {
    let params = params!{
        "id" => talent.id,
        "name" => talent.name,
        "slug" => talent.slug,
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
            id = VALUES(id),
            `name` = VALUES(`name`),
            slug = VALUES(slug),
            tier = VALUES(tier),
            active = VALUES(active),
            faction = VALUES(faction),
            championship = VALUES(championship),
            `show` = VALUES(`show`),
            image = VALUES(image),
            bio = VALUES(bio)";

    pool.prep_exec(query, params);

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