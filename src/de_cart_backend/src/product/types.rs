use candid::{ Deserialize, CandidType };


#[derive(Clone, Debug, Default, Deserialize, CandidType)]
pub struct Product {
    sku: String,
    merchant_id : String,
    name: String,
    price: candid::Nat,
    description: String,
    image_url: String
}