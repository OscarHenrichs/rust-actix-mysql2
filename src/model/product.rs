use serde::{Serialize, Deserialize};
 
// pub fn today() -> NaiveDate {
//     let l = Local::today();
 
//     NaiveDate::from_ymd(l.year(), l.month(), l.day())
// }
#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
    pub code: String,
    pub product_name: String
}