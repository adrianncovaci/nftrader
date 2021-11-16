use crate::schema::images;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub hashed_value: String,
}

#[derive(Insertable)]
#[table_name = "images"]
pub struct NewImage<'a> {
    pub hashed_value: &'a str,
}
