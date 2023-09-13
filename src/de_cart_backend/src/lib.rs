mod merchant;
mod cart;
mod product;
mod api;

use std::cell::RefCell;
use std::collections::HashMap;
use merchant::types::Merchant;
use ic_cdk;
use ic_cdk::export_candid;

/*
For now the users will remain anon. Each cart will have an associated
principle id /wallet that has permission to manipulate items in it via
access control
 */
thread_local!{
    static MERCHANTS : RefCell<Vec<Merchant>> = RefCell::new(vec![]);
    static CARTS : RefCell<HashMap<String, String>> = RefCell::default();

    // Can make the list of products public but encrypt sensitive data ?
    static PRODUCTS : RefCell<HashMap<String, String>> = RefCell::default();
}

export_candid!();