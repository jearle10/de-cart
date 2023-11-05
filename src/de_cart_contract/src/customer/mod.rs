use candid::{ CandidType, Deserialize };


#[derive(Deserialize, CandidType,Default)]
pub struct Customer {
    id : String ,
    email : String ,
    name : String,
    balance : String,
    address: String
}