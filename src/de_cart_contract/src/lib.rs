#![allow(unused_imports)]

mod customer;
mod merchant;
mod marketplace;
mod order;
mod product;
mod cart;
mod state;
// mod api;

use marketplace::Marketplace;
use customer::*;
use merchant::*;
use order::Order;
use product::*;
use cart::*;
use std::cell::RefCell;
use ic_cdk;
use ic_cdk::export_candid;


/*
For now the users will remain anon. Each cart will have an associated
principle id /wallet that has permission to manipulate items in it via
access control
 */
thread_local! {
    static STATE : RefCell<Marketplace> = RefCell::new(Marketplace::default())


    // 48GiB of Stable memory in a canister currently
    // Define some sensible cansiter limits to ensure upgrades possible
}

#[ic_cdk::query]
fn test() -> String {
    ic_cdk::println!("{}", ic_cdk::caller().to_text());
    "Hello from de-cart canister".to_string()
}

export_candid!();