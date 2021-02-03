#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub pract_date: String,
    pub title: String,
    pub goal: String,
    pub notes: String,
    pub pract_time: i32,
    pub focus_time: i32,
}

use super::schema::entries;

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry<'a> {
    pub pract_date: &'a str,
    pub title: &'a str,
    pub goal: &'a str,
    pub notes: &'a str,
    pub pract_time: &'a i32,
    pub focus_time: &'a i32,
}
