#[macro_use]
extern crate actix_web;
extern crate jsonwebtoken;
extern crate dotenv;
extern crate env_logger;

mod routes;
mod model;
mod controller;

use routes::product::{ get_product, query_product, create_product};
 
use mysql::*;
use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use actix_service::Service;
use std::default::Default;
use std::{env, io};

 
#[actix_rt::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let url = "mysql://root:Vegeta94@@localhost:3306/fisio_check_dev";
     
    let pool = match Pool::new(url) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to open DB connection. {:?}", e); return;
        }
    };
 
    let shared_data = actix_web::web::Data::new(pool);
 
    let server = match HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
                )
            .app_data(shared_data.clone())
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