use actix_web::{Error, web, middleware, App, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use actix_web::dev::ServiceRequest;
use diesel::r2d2::{self, ConnectionManager};
#[macro_use]
extern crate diesel;
extern crate dotenv;

// fn main() {
//     println!("Hello")
// }

 // this module will contain our handlers
mod errors;
mod user_handlers;
mod article_handlers;
mod models; // Models for our data base
mod schema; // Models for state (and then database)
mod auth;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[actix_rt::main]   // This provides a run-time for Actix actors, that will schedule,
                    // and run them. We use the actix_rt::main `attribute`, to signify
                    // this.
async fn main() -> std::io::Result<()> {
    // This returns a Result type
    // This async/await construct yield control from a current thread,
    // to some other thread, that will run, while the current one blocks.
    dotenv::dotenv().ok(); // Activate dotenv as early in app as possible
    std::env::set_var("Rust_Log", "actix_web=debug");

    let database_url = std::env::var("DATABASE_URL").expect("Url must be set.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "5000".to_string());

    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {   // The HttpServer type, manages HTTP requests.
                                // It accepts an application `factory`, which must be
                                // Send + Sync.
        let auth = HttpAuthentication::bearer(validate);
        App::new()
            .data(pool.clone())
            // .route("/home")
            // .route("/login")
            // .route("/logout")
            .service(web::scope("/api")
                .route("/articles", web::get().to(article_handlers::get_articles))
                .route("/articles/{id}", web::get().to(article_handlers::get_article_by_id))
                .route("/articles", web::post().to(article_handlers::add_article))
                .route("/articles/{id}", web::delete().to(article_handlers::delete_article))
                .route("/users", web::get().to(user_handlers::get_users))
                .route("/users/{id}", web::get().to(user_handlers::get_user_by_id))
                .route("/users", web::post().to(user_handlers::add_user))
                .route("/users/{id}", web::delete().to(user_handlers::delete_user)))   
    }).bind("127.0.0.1:8080")?  // ? Bubbles up errors from the associated function.                       // Bind attaches a socket address to the application.
    .run() // Returns an instance of Server type.
    .await
}

async fn validate(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            }
            else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into())
    }
}