use ic_cdk::export::candid::{ candid_method, Deserialize };
use std::cell::{Ref, RefCell};
use candid::CandidType;


/*

1. Need to break out the methods into this file into three canisters
- Merchant
- Product
- Cart

2. Need to work out how to generate candid UI automatically from methods

3. Add tests
 */

#[derive(Clone, Debug, Default, Deserialize, CandidType)]
struct Merchant {
    id : candid::Nat, // This should be the wallet id of the merchant
    products : Vec<candid::Nat> // List of product ids
}

#[derive(Clone, Debug, Default, Deserialize, CandidType)]
struct Product {
    sku: String,
    name: String,
    price: candid::Nat,
    image_url: String
}

thread_local!{ static MERCHANTS : RefCell<Vec<Merchant>> = RefCell::new(vec![]) }

#[ic_cdk::update]
#[candid_method(update)]
fn add_merchant(id : candid::Nat) {
    MERCHANTS.with(|merchants| {
       merchants
           .borrow_mut()
           .insert(0, Merchant::default())
    });
}

#[ic_cdk::query]
fn get_merchant(id: candid::Nat) -> Merchant {
    MERCHANTS.with(|merchants| {
        merchants
            .borrow()
            .first()
            .unwrap()
            .clone()
    })
}


#[ic_cdk::query]
#[candid_method(query)]
fn get_self() -> String {
    let id = ic_cdk::api::caller();
    format!("Cannister was called by {}" , id)
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