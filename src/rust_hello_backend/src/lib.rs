use ic_cdk::export::candid::{ candid_method };
use std::cell::{Ref, RefCell};

thread_local!{ static MERCHANTS : RefCell<Vec<candid::Nat>> = RefCell::new(vec![]) }

#[ic_cdk::update]
#[candid_method(update)]
fn add_merchant(id : candid::Nat) {
    MERCHANTS.with(|merchants| {
       merchants
           .borrow_mut()
           .insert(0, id)
    });
}

#[ic_cdk::query]
fn get_merchant(id: candid::Nat) -> candid::Nat {
    MERCHANTS.with(|merchants| {
        merchants
            .borrow()
            .first()
            .unwrap()
            .clone()
    })
}

#[ic_cdk::update]
#[candid_method(update)]
fn remove_merchant() -> String {
    format!("Removed a merchant from the platform")
}


#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
/* Should probably have separate canisters for
merchant functions and user TBC on design
 */

// User functions
#[ic_cdk::query]
fn add_to_cart() -> String {
    format!("Added product to cart")
}

#[ic_cdk::query]
fn delete_from_cart() -> String {
    format!("Remove product from cart")
}

#[ic_cdk::query]
fn add_product() -> String {
    format!("Remove product from cart")
}

// Merchant functions
#[ic_cdk::query]
fn remove_product() -> String {
    format!("Listed product on de-cart")
}

#[ic_cdk::query]
fn import_products() -> String {
    format!("Fetched products from e-commerce platform")
}

#[test]
fn check_candid_interface() {
    use candid::utils::{service_compatible, CandidSource};
    use std::path::Path;

    candid::export_service!();
    let new_interface = __export_service();

    service_compatible(
        CandidSource::Text(&new_interface),
        CandidSource::File(Path::new("rust_hello_backend.did")),
    ).unwrap();
}