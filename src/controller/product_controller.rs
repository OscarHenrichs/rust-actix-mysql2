use crate::models::product::Product;
use r2d2::{PooledConnection, ManageConnection};
use r2d2_mysql::mysql::{QueryResult, from_row, PooledConn};
use r2d2_mysql::mysql::prelude::*;
use r2d2_mysql::mysql::params;

// pub fn insert_product(
//     conn: &mut PooledConn, 
//     product: &Product) -> std::result::Result<u64, mysql::error::Error> {
 
//     conn.exec_drop(
//         "insert into PRODUCT (product_code, price, name) values (:product_code, :price, :name, :last_update)",
//         params! {
//             "product_code" => &product.code,
//             "name" => &product.product_name,
//         },
//     )
//     .and_then(|_| Ok(conn.last_insert_id()))
// }

pub fn find_product_in_price_range(
    conn: &mut r2d2::PooledConnection<r2d2_mysql::MysqlConnectionManager>,
    price_from: f32,
    price_to: f32) -> std::result::Result<Vec<Product>, r2d2_mysql::mysql::error::Error> {
    conn.exec_map(
        "select product_id, product_code, name from PRODUCT where price>=:price_from and price <=:price_to",
        params! {
            "price_from" => price_from,
            "price_to" => price_to,
        },
        |(product_id, product_code, name)| Product {
            id: product_id,
            code: product_code,
            product_name: name
        }
    )
}

// pub fn find_product_by_id(
//     conn: &mut PooledConn,
//     product_id: u64,
// ) -> std::result::Result<Option<Product>, mysql::error::Error> {
//     let row = conn.exec_first(
//         "select product_id, product_code, name from PRODUCT where product_id=:product_id",
//         params! {
//             "product_id" => product_id
//         },
//     )?;
 
//     Ok(row.map(|(product_id, product_code, name, )| Product {
//         id: product_id,
//         code: product_code,
//         product_name: name,
//     }))
// }