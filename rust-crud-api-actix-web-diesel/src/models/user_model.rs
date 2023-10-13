use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug,Serialize,Deserialize, Queryable)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub email: String,
}


#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

