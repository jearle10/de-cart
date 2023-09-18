pub (crate) mod types;

use std::collections::HashMap;
use crate::{api, product};
use crate::merchant::types::Merchant;
use product::types::Product;
use crate::state::{ProductList, State};

use super::PRODUCTS;
use super::STATE;

#[ic_cdk::update]
fn add_product(item : Product) -> Option<Product>{
    STATE.with(|state| {
        state
            .borrow_mut() // RefCell -> State
            .products
            .as_mut() // Box<ProductStore> -> ProductStore
            .add(item)
    })
}

#[ic_cdk::query]
fn get_product(merchant_id : String , id : String) -> Option<Product>{
    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);

    STATE.with(|state| {
        state
            .borrow() // RefCell -> State
            .products
            .as_ref() // Box<ProductStore> -> ProductStore
            .get(merchant_id , id)
    })
}

#[ic_cdk::query]
fn list_products(merchant_id : String) -> Option<ProductList> {
    // let principal = ic_cdk::caller();
    // ic_cdk::println!("{}", principal);
    STATE.with(|state| {
        state
            .borrow()
            .products
            .as_ref()
            .get_all(merchant_id)
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
fn remove_product(merchant_id : String , id : String) -> Option<Product> {
    STATE.with(|state| {
        state
            .borrow_mut() // RefCell -> State
            .products
            .as_mut() // Box<ProductStore> -> ProductStore
            .delete(merchant_id , id)
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