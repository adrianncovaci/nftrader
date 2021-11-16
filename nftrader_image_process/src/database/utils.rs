use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE url must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to database {}", database_url))
}
