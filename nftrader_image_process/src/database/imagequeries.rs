use crate::models::image::Image;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn get_image(connection: &PgConnection, hashed_value_req: &str) -> Option<Image> {
    use crate::schema::images::dsl::*;

    let _images = images
        .filter(hashed_value.eq(hashed_value_req))
        .limit(1)
        .load::<Image>(connection)
        .expect("Error loading image");

    _images.into_iter().nth(0)
}
