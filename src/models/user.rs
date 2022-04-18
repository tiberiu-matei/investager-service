use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "itg_user"]
pub struct CreateUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
}
