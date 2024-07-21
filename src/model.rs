use diesel::prelude::*;

use crate::schema::anime;

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = anime)]
pub struct Anime {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub mal_id: i32,
    pub al_id: i32,
    pub japanese_title: Option<String>,
    pub image: String,
    pub type_: String,
    pub sub_or_dub: String,
}
