mod customer;
mod merchant;
mod marketplace;
mod order;
mod product;
mod cart;
mod state;
// mod api;

use marketplace::Marketplace;
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

// #[ic_cdk::query]
// fn test() -> product::ProductStore {
//     let marketplace = STATE.take();
//     marketplace.products
// }

export_candid!();