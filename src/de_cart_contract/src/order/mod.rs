use candid::{ Deserialize , CandidType};

// use crate::product::P

#[derive(Deserialize, CandidType)]
pub struct Order {
    id : String , 
    status : String , 
    merchant_id : String ,
    customer_id : String ,
    amount : String ,
    tracking_number : String,
    carrier : String ,
    products: Vec<String>, // Todo update to Product struct,
    address : String
}