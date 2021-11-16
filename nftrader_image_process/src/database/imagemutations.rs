use diesel::PgConnection;

use crate::models::image::{Image, NewImage};

pub fn create_image<'a>(connection: &PgConnection, hashed: &'a str) -> Image {
    use crate::diesel::RunQueryDsl;
    use crate::schema::images;

    let new_image = NewImage {
        hashed_value: hashed,
    };

    diesel::insert_into(images::table)
        .values(&new_image)
        .get_result(connection)
        .expect("Error saving image")
}
