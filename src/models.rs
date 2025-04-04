use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::url_map)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UrlMap {
    pub id: i32,
    pub original_url: String,
    pub tiny_url: String,
    pub fetch_count: i32,
}