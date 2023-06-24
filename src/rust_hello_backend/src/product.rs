use ic_cdk::export::candid::{ candid_method, Deserialize, CandidType };

#[derive(Clone, Debug, Default, Deserialize, CandidType)]
struct Product {
    sku: String,
    name: String,
    price: candid::Nat,
    image_url: String
}

#[ic_cdk::update]
#[candid_method(update)]
fn add_product() -> String {
    format!("Listed product on de-cart")
}

#[ic_cdk::update]
#[candid_method(update)]
fn remove_product() -> String {
    format!("Removed product on de-cart")
}

#[ic_cdk::query]
#[candid_method(query)]
fn get_cscart_products() -> String {
    format!("Fetched cscart products")
}