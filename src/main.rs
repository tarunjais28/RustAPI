#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use call_center::routes;
use db::*;
use std::env;

mod call_center;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Read host usr from .env file
    let args: Vec<String> = env::args().collect();
    let host_url: &str = &args[1];
    let con_str: &str = &args[2];

    // Establish DB Pool
    let pool = establish_db_pool(con_str);
    println!("Starting server at: {}", &host_url);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(routes::agent_by_id)
            .service(routes::call_by_id)
            .service(routes::calls_by_agent)
            .service(routes::daily_call_volume)
    })
    .bind(&host_url)?
    .run()
    .await
}
