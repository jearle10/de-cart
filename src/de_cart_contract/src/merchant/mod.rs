pub(crate) mod types;
use candid::candid_method;
use ic_cdk;

use types::Merchant;
use super::MERCHANTS;

fn get_self() -> String {
    let id = ic_cdk::api::caller();
    format!("Cannister was called by {}" , id)
}

// #[ic_cdk::update]
fn register_merchant(_id : candid::Nat) {
    MERCHANTS.with(|merchants| {
        merchants
            .borrow_mut()
            .insert(0, Merchant::default())
    });
}

// #[ic_cdk::update]
fn deregister_merchant() -> String {
    format!("Removed a merchant from the platform")
}

// #[ic_cdk::update]
fn update_merchant(_id: candid::Nat) {
    format!("Updated merchant's details");
}

// #[ic_cdk::query]
fn get_merchant(_id: candid::Nat){
    format!("Retrieved merchant's details");
    // MERCHANTS.with(|merchants| {
    //     merchants
    //         .borrow()
    //         .first()
    //         .unwrap()
    //         .clone()
    // })
}

