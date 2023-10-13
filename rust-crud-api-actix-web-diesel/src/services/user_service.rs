use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use diesel::prelude::*;

use crate::models::user_model::{NewUser, User};
use crate::schema::users::{email, id, name};
use crate::schema::users::dsl::users;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn add_new_user(paylod: &NewUser, conn: &PgConnection) -> Result<User, DbError> {
    let res = diesel::insert_into(users)
        .values(paylod)
        .get_result(conn)?;

    Ok(res)
}

pub fn find_all(conn: &PgConnection) -> Result<Vec<User>, DbError> {
    let items = users.load::<User>(conn)?;
    Ok(items)
}

pub fn find_by_id(user_id: i32, conn: &PgConnection) -> Result<Option<User>, DbError> {
    let user = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn update_user(user_id: i32, payload: &NewUser, conn: &PgConnection) -> Result<User, DbError> {
    let user = diesel::update(users.find(user_id))
        .set((name.eq(&payload.name), email.eq(&payload.email)))
        .get_result::<User>(conn)?;

    Ok(user)
}

pub fn delete_user(user_id:i32, conn: &PgConnection) -> Result<usize,DbError>{
    let count = diesel::delete(users.find(user_id)).execute(conn)?;
    Ok(count)
}