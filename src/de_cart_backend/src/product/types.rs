use candid::{ Deserialize, CandidType };


#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct Product {
    sku: String,
    merchant_id : String,
    name: String,
    price: candid::Nat,
    description: String,
    image_url: String
}

impl Default for Product {
    fn default() -> Self {
        Product {
            sku: "default".to_string(),
            merchant_id: "default_merchant".to_string(),
            name: "".to_string(),
            price: Default::default(),
            description: "".to_string(),
            image_url: "".to_string(),
        }
    }
}