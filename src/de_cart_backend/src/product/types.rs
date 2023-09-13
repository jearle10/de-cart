use candid::{ Deserialize, CandidType };


#[derive(Clone, Debug, Default, Deserialize, CandidType)]
pub struct Product {
    sku: String,
    name: String,
    price: candid::Nat,
    image_url: String
}
