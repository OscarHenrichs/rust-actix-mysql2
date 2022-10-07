#[macro_use]
extern crate log;
extern crate actix_web;
extern crate serde_json;
extern crate actix_cors;
extern crate actix_rt;
extern crate bcrypt;
extern crate derive_more;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate jsonwebtoken;
extern crate serde;
extern crate uuid;
extern crate r2d2;
extern crate r2d2_mysql;

mod routes;
mod models;
mod controller;
// mod middleware;
// mod utils;
mod config;

use routes::product::{ get_product, query_product, create_product};

use std::thread;


 
use mysql::*;
use actix_web::*;
use actix_cors::Cors;
use dotenv::dotenv;
use std::{env};
 
#[actix_rt::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    // let pool = config::db::config_db(&db_url);

    let app_data = web::Data::new(config::db::AppState {
        app_name: String::from("turreta"),
        pool: config::db::get_pool(&db_url).unwrap(),
    });
 
    let server = match HttpServer::new(move || {
        App::new()
            .wrap(
            Cors::default() // allowed_origin return access-control-allow-origin: * by default
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
            )
            .app_data(app_data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(get_product)
            .service(query_product)
            .service(create_product)
    }).bind("127.0.0.1:8080") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind port. {:?}", e);
            return;
        }
    };
 
    match server.run().await {
        Ok(_) => println!("Server exited normally."),
        Err(e) => println!("Server exited with error: {:?}", e),
    };
}