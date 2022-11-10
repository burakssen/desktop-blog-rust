use super::schema::user;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password: String,
    pub mail: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub user_name: String,
    pub password: String,
    pub mail: String,
}
