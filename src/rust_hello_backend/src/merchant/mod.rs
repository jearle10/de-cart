pub(crate) mod types;

use ic_cdk::export::candid::{candid_method, Deserialize, CandidType };
use std::cell::RefCell;
use types::Merchant;
use super::MERCHANTS;

#[ic_cdk::query]
fn get_self() -> String {
    let id = ic_cdk::api::caller();
    format!("Cannister was called by {}" , id)
}

#[ic_cdk::update]
#[candid_method(update)]
fn register_merchant(id : candid::Nat) {
    MERCHANTS.with(|merchants| {
        merchants
            .borrow_mut()
            .insert(0, Merchant::default())
    });
}

#[ic_cdk::update]
#[candid_method(update)]
fn deregister_merchant() -> String {
    format!("Removed a merchant from the platform")
}

#[ic_cdk::update]
#[candid_method(query)]
fn update_merchant(id: candid::Nat) {
    format!("Updated merchant's details");
}

#[ic_cdk::query]
#[candid_method(query)]
fn get_merchant(id: candid::Nat){
    format!("Retrieved merchant's details");
    // MERCHANTS.with(|merchants| {
    //     merchants
    //         .borrow()
    //         .first()
    //         .unwrap()
    //         .clone()
    // })
}

