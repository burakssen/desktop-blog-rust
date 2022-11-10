pub(crate) mod models;
mod schema;

use models::{NewUser, User};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_all_users() -> String {
    use self::schema::user::dsl::*;

    let connection = &mut establish_connection();
    let results = user.load::<User>(connection).expect("Error loading users");
    match serde_json::to_string(&results) {
        Ok(val) => val,
        Err(e) => String::from(e.to_string()),
    }
}

pub fn create_user(user_name: String, password: String, mail: String) -> User {
    use self::schema::user;

    let connection = &mut establish_connection();

    let new_user = NewUser {
        user_name,
        password,
        mail,
    };

    diesel::insert_into(user::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user")
}
