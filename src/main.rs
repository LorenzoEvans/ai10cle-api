use actix_web::{dev::ServiceRequest, Error, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
#[macro_use]
extern crate diesel;
extern crate dotenv;

// fn main() {
//     println!("Hello")
// }


mod handlers; // this module will contain our handlers
mod errors;
mod models; // Models for our data base
mod schema; // Models for state (and then database)
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // This returns a Result type
    dotenv::dotenv().ok(); // Activate dotenv as early in app as possible
    std::env::set_var("Rust_Log", "actix_web=debug");

    let database_url = std::env::var("DATABASE_URL").expect("Url must be set.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))   
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}