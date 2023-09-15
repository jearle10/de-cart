mod merchant;
mod cart;
mod product;
mod api;
mod state;

use std::cell::RefCell;
use std::collections::HashMap;
use merchant::types::Merchant;
use product::types::Product;
use state::ProductList;
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
    static PRODUCTS : RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());

    // Update version of state using combined struct
    static STATE : RefCell<state::State> = RefCell::new(state::State::default())
}

export_candid!();