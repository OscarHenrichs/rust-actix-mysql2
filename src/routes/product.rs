// use crate::controller::product_controller::find_product_by_id;
// use crate::controller::product_controller::insert_product;
use crate::controller::product_controller::find_product_in_price_range;
use crate::models::product::Product;
use crate::config::db::AppState;
use actix_web::*;
use r2d2_mysql::mysql::prelude::Queryable;
use r2d2_mysql::mysql::{QueryResult, from_row, PooledConn};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct ProductQuery {
    price_from: f32,
    price_to: f32,
}
 
#[get("/product")]
pub async fn query_product(query: web::Query<ProductQuery>, data: web::Data<AppState>) -> HttpResponse {

    let app_name = &data.app_name; // <- get app_name
    let pool = &data.pool;
    let pool = pool.clone();
    // let mut conn = pool.get().map_err(|err| {
    //     println!(
    //         "get connection from pool error in line:{} ! error: {:?}",
    //         line!(),
    //         err
    //     )
    // }).unwrap();
 
    // let param = info.into_inner();
    // let qr = conn.exec_iter("select person_id, person_name from person where person_id = ?", (param, )).unwrap();
 
    // let mut rec: Option<(i32, String)> = None;
 
    // for row in qr {
    //     rec = Some(from_row(row.unwrap()));
    //     break;
    // }
 
    // let unwrap_rec = rec.unwrap();
    // format!("Hello {} ({})! \n from {}",  unwrap_rec.1, unwrap_rec.0, app_name)
    match pool.get().map_err(|err| {
        println!(
            "get connection from pool error in line:{} ! error: {:?}",
            line!(),
            err
        )
    }).and_then(|mut conn: r2d2::PooledConnection<r2d2_mysql::MysqlConnectionManager>| find_product_in_price_range(&mut conn, query.price_from, query.price_to))
    {
        Ok(result_list) => HttpResponse::Ok().json(result_list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// #[get("/product/{id}")]
// async fn get_product(path: web::Path<u64>, data: web::Data<AppState>) -> HttpResponse {
// //     let product_id = *path;
 
// //     match data
// //         .get_conn()
// //         .and_then(|mut conn| find_product_by_id(&mut conn, product_id))
// //     {
// //         Ok(res) => match res {
// //             Some(product) => HttpResponse::Ok().json(product),
// //             None => HttpResponse::NotFound().finish(),
// //         },
// //         Err(_) => HttpResponse::InternalServerError().finish(),
// //     }
// }


// #[post("/product")]
// async fn create_product(product_json: web::Json<Product>, data: web::Data<AppState>) -> HttpResponse {
//     // let product = product_json.into_inner();
 
//     // match data
//     //     .get_conn()
//     //     .and_then(|mut conn| insert_product(&mut conn, &product))
//     // {
//     //     Ok(product_id) => {
//     //         //Return a Product with id set.
//     //         HttpResponse::Ok().json(Product {
//     //             id: product_id,
//     //             ..product
//     //         })
//     //     },
//     //     Err(_) => HttpResponse::InternalServerError().finish(),
//     // }
// }