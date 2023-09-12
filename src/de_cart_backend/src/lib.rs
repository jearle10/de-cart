mod merchant;
mod cart;
mod product;
mod api;

use std::cell::RefCell;
use std::collections::HashMap;
use merchant::types::Merchant;

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

#[test]
fn check_candid_interface() {
    use candid::utils::{service_compatible, CandidSource };
    use std::path::Path;
    

    // Generate the service definition (candid format)
    candid::export_service!();
    let new_interface = __export_service();

    // Write the generated service definition to a file for dfx deploy
    let buffer = new_interface.as_bytes();
    std::fs::write("backend.did", buffer)
        .expect("Issue generating backend.did file");

    service_compatible(
        CandidSource::Text(&new_interface),
        CandidSource::File(Path::new("backend.did")),
    ).unwrap();
}