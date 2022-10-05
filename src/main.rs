mod routes;
mod model;
mod controller;

use routes::product::{ get_product, query_product, create_product};
 
use mysql::*;
use actix_web::*;
 
#[actix_rt::main]
async fn main() {
    let url = "mysql://root:Vegeta94@localhost:3306/fisio_check_dev";
     
    let pool = match Pool::new(url) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to open DB connection. {:?}", e); return;
        }
    };
 
    let shared_data = web::Data::new(pool);
 
    let server = match HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
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