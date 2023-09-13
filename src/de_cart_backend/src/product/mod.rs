pub (crate) mod types;
use crate::api;

// #[ic_cdk::update]
fn list_product() -> String {
    format!("Listed product on de-cart")
}

#[ic_cdk::update]
fn delist_product() -> String {
    format!("Removed product on de-cart")
}

// #[ic_cdk::query]
fn list_products() -> String {
    format!("Listed products")
}

#[ic_cdk::query]
fn get_product() -> String {
    api::Shopify::api()
}

#[ic_cdk::update]
async fn import_products(
    access_token : String,
    host : String
) -> String {
    api::Shopify::get_all_products(access_token, host).await;
    // ic_cdk::println!("Testing printing logs");
    format!("Fetched cscart products")
}