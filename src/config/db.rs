#[allow(unused_imports)]
// use mysql::*;
// use mysql::{Opts,OptsBuilder};
use r2d2_mysql::mysql::{Opts, OptsBuilder, QueryResult, from_row};
use std::sync::Arc;
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;
// use std::env;
pub struct AppState {
    pub app_name: String,
    pub pool: Arc<Pool<MysqlConnectionManager>>,
}

pub fn get_pool(url: &str) -> Option<Arc<Pool<MysqlConnectionManager>>> {
    let opts = Opts::from_url(&url).unwrap();
    let o = OptsBuilder::from_opts(opts);

    let manager = r2d2_mysql::MysqlConnectionManager::new(o);
 
    println!("Getting pool");
 
    let pool = Arc::new(r2d2::Pool::new(manager).unwrap());
    return Option::Some(pool);
}

// pub fn config_db(url: &str) -> Pool {
//     info!("configuring database...");

//     let manager = match Pool::new(url) {
//         Ok(pool) => pool,
//         Err(e) => {
//             println!("Failed to open DB connection. {:?}", e); return;
//         }
//     };
//     let pool = r2d2::Pool::builder()
//         .build(manager)
//         .expect("Failed to create pool.");

//     pool
// }