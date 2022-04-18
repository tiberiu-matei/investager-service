#[macro_use]
extern crate diesel;
extern crate dotenv;

mod handlers;
mod models;
mod schema;

use actix_web::{get, web, App, HttpServer, Responder};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

// use self::models::{CreateUser, User};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[get("/{id}/{name}")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/user/all", web::get().to(handlers::user::get_all))
            .route("/user/{id}", web::get().to(handlers::user::get_by_id))
            .route("/user", web::post().to(handlers::user::create))
            .route("/user/{id}", web::delete().to(handlers::user::delete))
    })
    .bind("127.0.0.1:7331")?
    .run()
    .await
}
