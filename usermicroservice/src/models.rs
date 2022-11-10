use crate::schema::user;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password: String,
    pub mail: String,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
    pub mail: &'a str,
}
