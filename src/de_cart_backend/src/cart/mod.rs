use ic_cdk::export::candid::{ candid_method };

#[ic_cdk::update]
#[candid_method(update)]
fn add_to_cart() -> String {
    format!("Added product to cart")
}

#[ic_cdk::update]
#[candid_method(update)]
fn delete_from_cart() -> String {
    format!("Remove product from cart")
}

#[ic_cdk::update]
#[candid_method(query)]
fn get_cart() -> String {
    format!("Retrieved cart ")
}