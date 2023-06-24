use ic_cdk::export::candid::{ candid_method, Deserialize, CandidType };
use std::cell::RefCell;

#[derive(Clone, Debug, Default, Deserialize, CandidType)]
pub struct Merchant {
    id : candid::Nat, // This should be the wallet id of the merchant
    products : Vec<candid::Nat> // List of product ids
}

thread_local!{ static MERCHANTS : RefCell<Vec<Merchant>> = RefCell::new(vec![]) }

#[ic_cdk::query]
fn get_self() -> String {
    let id = ic_cdk::api::caller();
    format!("Cannister was called by {}" , id)
}

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
#[candid_method(query)]
fn get_merchant(id: candid::Nat) -> Merchant {
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
