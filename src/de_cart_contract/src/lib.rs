mod customer;
mod merchant;
mod marketplace;
mod order;
mod product;
mod cart;
mod state;
// mod api;
use product::*;
use std::cell::RefCell;
use ic_cdk;
use ic_cdk::export_candid;

/*
For now the users will remain anon. Each cart will have an associated
principle id /wallet that has permission to manipulate items in it via
access control
 */
thread_local! {
    static STATE : RefCell<marketplace::Marketplace> = RefCell::new(marketplace::Marketplace::default())
}

#[ic_cdk::query]
fn test() -> product::ProductStore {
    let marketplace = STATE.take();
    marketplace.products
}

export_candid!();