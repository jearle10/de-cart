#![allow(unused_imports)]
// #![feature(test)]

mod customer;
mod merchant;
mod marketplace;
mod order;
mod product;
mod cart;
mod state;
// mod api;

use candid::{CandidType, Deserialize};
use ic_cdk::api::stable::WASM_PAGE_SIZE_IN_BYTES;
use marketplace::Marketplace;
use customer::*;
use merchant::*;
use order::Order;
use product::*;
use cart::*;
use std::cell::RefCell;
use ic_cdk;
use std::mem;
use ic_cdk::export_candid;


/*
For now the users will remain anon. Each cart will have an associated
principle id /wallet that has permission to manipulate items in it via
access control
 */
thread_local! {
    static STATE : RefCell<Marketplace> = RefCell::new(Marketplace::default())


    // 96GiB of Stable memory in a canister currently
    // 4GiB heap size
    // Define some sensible cansiter limits to ensure upgrades possible
}

#[derive(Deserialize, CandidType)]
struct MarketplaceStats {
    total : String,
    customers : String,
    merchants : String,
    products : String,
    carts : String,
    orders : String
}


#[ic_cdk::query]
fn marketplace_stats() -> MarketplaceStats {
    ic_cdk::println!("{}", ic_cdk::caller().to_text());

    let marketplace = STATE.take();

    let mut bytes = 0;

    let customers_size =  marketplace.customers.total_bytes();
    let merchants_size = marketplace.merchants.total_bytes();
    let products_size = marketplace.products.total_bytes();
    let carts_size = marketplace.carts.total_bytes();
    let orders_size =  marketplace.orders.total_bytes();

    let customers_count =  marketplace.customers.total_customer();
    let merchants_count = marketplace.merchants.total_merchants();
    let products_count = marketplace.products.total_products();
    let carts_count = marketplace.carts.total_carts();
    let orders_count =  marketplace.orders.total_orders();

    bytes += customers_size + merchants_size + products_size + carts_size + orders_size;

    let count = marketplace.products.total_products();
    STATE.set(marketplace);

    let stats = MarketplaceStats { 
        total: format!("bytes: {}", bytes), 
        customers: format!("count: {}, size: {}", customers_count, customers_size),  
        merchants: format!("count: {}, size: {} ", merchants_count, merchants_size),  
        products: format!("count: {}, size: {}", products_count, products_size), 
        carts: format!("count: {}, size: {}", carts_count, carts_size),
        orders: format!("count: {}, size: {}", orders_count, orders_size)  
    };
    stats
}

export_candid!();