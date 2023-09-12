use ic_cdk::export::candid::{Deserialize, CandidType };

#[derive(Clone, Debug, Default, Deserialize, CandidType)]
pub struct Merchant {
    pub id : candid::Nat, // This should be the wallet id of the merchant
    pub products : Vec<candid::Nat> // List of product ids
}