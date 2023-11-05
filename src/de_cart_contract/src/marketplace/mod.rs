use candid::{ Deserialize, CandidType };
use std::collections::HashMap;

use crate::customer;
use crate::merchant;
use crate::order;
use crate::product;
use crate::cart;

#[derive(Deserialize, CandidType)]
pub struct Marketplace {
    customers : HashMap<String , customer::Customer>,
    merchants : HashMap<String , merchant::Merchant>,
    orders : HashMap<String, order::Order>,
    products: HashMap<String, product::Product>,
    carts : HashMap<String , cart::Cart>
}

impl Default for Marketplace {
    fn default() -> Self {
        Self {
            customers : HashMap::new(),
            merchants: HashMap::new(),
            orders: HashMap::new(),
            products: HashMap::new(),
            carts: HashMap::new(),
        }
    }
}