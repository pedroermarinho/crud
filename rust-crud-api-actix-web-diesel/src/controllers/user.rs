use actix_web::*;
use crate::DbPool;
use crate::models::user_model::NewUser;
use crate::services::user_service;


#[get("/users")]
async fn get_users(
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = pool.get()?;
        let result = user_service::find_all(&conn);
        result
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn get_user(
    id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        let result = user_service::find_by_id(id.into_inner(), &conn);
        result
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create_user(
    pool: web::Data<DbPool>,
    payload: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        let result = user_service::add_new_user(&payload, &conn);
        result
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}

#[put("/users/{id}")]
async fn update_user(
    id: web::Path<i32>,
    payload: web::Json<NewUser>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        let result = user_service::update_user(id.into_inner(),&payload, &conn);
        result
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete_user(
    id: web::Path<i32>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let  result = web::block(move || {
        let conn = pool.get()?;
        let result = user_service::delete_user(id.into_inner(),&conn);
        result
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(format!("{}",result)))
}

