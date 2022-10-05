use super::db_layer::*;
use actix_web::*;
use mysql::*;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct ProductQuery {
    price_from: f32,
    price_to: f32,
}
 
#[get("/product")]
pub async fn query_product(query: web::Query<ProductQuery>, data: web::Data<Pool>) -> HttpResponse {
    match data
        .get_conn()
        .and_then(|mut conn| find_product_in_price_range(&mut conn, query.price_from, query.price_to))
    {
        Ok(result_list) => HttpResponse::Ok().json(result_list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/product/{id}")]
async fn get_product(path: web::Path<u64>, data: web::Data<Pool>) -> HttpResponse {
    let product_id = *path;
 
    match data
        .get_conn()
        .and_then(|mut conn| find_product_by_id(&mut conn, product_id))
    {
        Ok(res) => match res {
            Some(product) => HttpResponse::Ok().json(product),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[post("/product")]
async fn create_product(product_json: web::Json<Product>, data: web::Data<Pool>) -> HttpResponse {
    let product = product_json.into_inner();
 
    match data
        .get_conn()
        .and_then(|mut conn| insert_product(&mut conn, &product))
    {
        Ok(product_id) => {
            //Return a Product with id set.
            HttpResponse::Ok().json(Product {
                id: product_id,
                ..product
            })
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}