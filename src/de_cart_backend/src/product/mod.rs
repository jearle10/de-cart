pub (crate) mod types;

use std::collections::HashMap;
use crate::{api, product};
use crate::merchant::types::Merchant;
use product::types::Product;
use crate::state::{ProductList, State};

use super::PRODUCTS;
use super::STATE;

#[ic_cdk::update]
fn add_product(item : Product){
    STATE.with(|state| {
        state
            .borrow_mut() // RefCell -> State
            .products
            .as_mut() // Box<ProductStore> -> ProductStore
            .add(item);
    });
}

#[ic_cdk::query]
fn get_product(id : String) -> Option<Product> {
    STATE.with(|state| {
        state
            .borrow() // RefCell -> State
            .products
            .as_ref() // Box<ProductStore> -> ProductStore
            .get(id)
    })
}

#[ic_cdk::query]
fn list_products() -> HashMap<String , Product> {
    let principal = ic_cdk::caller();
    ic_cdk::println!("{}", principal);
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.products.as_mut().get_all().unwrap_or(HashMap::new())
    })
}


#[ic_cdk::update]
fn update_product(item : Product){
    STATE.with(|state| {
        state
            .borrow_mut() // RefCell -> State
            .products
            .as_mut() // Box<ProductStore> -> ProductStore
            .update(item);
    });
}

#[ic_cdk::update]
fn remove_product(id : String) -> Option<Product> {
    STATE.with(|state| {
        state
            .borrow_mut() // RefCell -> State
            .products
            .as_mut() // Box<ProductStore> -> ProductStore
            .delete(id)
    })
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