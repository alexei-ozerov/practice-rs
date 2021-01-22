#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub title: String,
    pub body: String,
}

use super::schema::entries;

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
