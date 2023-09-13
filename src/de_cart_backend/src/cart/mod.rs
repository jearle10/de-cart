use candid::candid_method;
use ic_cdk;

// #[ic_cdk::update]
fn add_to_cart() -> String {
    format!("Added product to cart")
}

// #[ic_cdk::update]
fn delete_from_cart() -> String {
    format!("Remove product from cart")
}

// #[ic_cdk::update]
fn get_cart() -> String {
    format!("Retrieved cart ")
}