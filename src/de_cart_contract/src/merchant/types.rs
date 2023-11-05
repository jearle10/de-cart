use candid::{Deserialize, CandidType };

#[derive(Clone, Debug, Default, Deserialize, CandidType)]
pub struct Merchant {
    pub id : candid::Nat, // This should be the wallet id of the merchant,
    name: String,
    email : String
}