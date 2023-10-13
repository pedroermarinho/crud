#[macro_use]
extern crate diesel;

use actix_web::{middleware,App, HttpServer, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use controllers::about;
use controllers::user;

mod controllers;
mod models;
mod schema;
mod services;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(about::hello)
            .service(about::echo)
            .service(user::get_users)
            .service(user::get_user)
            .service(user::create_user)
            .service(user::delete_user)
            .service(user::update_user)
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}

