pub (crate) mod types;
use crate::api;
use crate::merchant::types::Merchant;
use crate::product::types::Product;
use crate::state::State;

use super::PRODUCTS;
use super::STATE;

#[ic_cdk::update]
fn add_default_product(){
    STATE.with(|state| {
        state
            .borrow_mut()
            .products
            .as_mut()
            .add(Product::default());
    });
}

// #[ic_cdk::update]
fn delist_product() -> String {
    format!("Removed product on de-cart")
}

#[ic_cdk::query]
fn list_products() -> String {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        ic_cdk::println!("{:#?}", state.products.as_mut().get_all())
    });
    format!("Listed products")
}

// #[ic_cdk::update]
fn get_product() -> String {
    format!("Retrieved product")
}

// #[ic_cdk::update]
async fn import_products(
    access_token : String,
    host : String
) -> String {
    api::Shopify::get_all_products(access_token, host).await;
    // ic_cdk::println!("Testing printing logs");
    format!("Fetched cscart products")
}