use std::collections::HashMap;

use candid::{ Deserialize, CandidType };
use crate::product;
use crate::state::Store;

type CustomerId = String;

#[derive(Deserialize, CandidType, Default)]
pub struct Cart {
    id : String ,
    user_id : String ,
    merchant_id : String, 
    items : Vec<CartItem>
}

#[derive(Deserialize, CandidType, Default)]
struct CartItem {
    product_id: String,
    qty : u32
}

pub struct CartStore {
    pub (crate) carts : HashMap<String, Cart>
}

impl CartStore {
    pub fn new() -> Self {
        Self {
            carts : HashMap::new()
        }
    }
}

impl Store <Cart> for CartStore {
    fn get(&self, customer_id : String ) -> Option<&Cart> {
        self.carts.get(&customer_id)
    }

    fn add(&mut self, cart : Cart) -> Option<Cart> {
        self.carts.insert(cart.merchant_id.clone(), cart)
    }

    fn get_all(&self) -> Vec<Cart> {
        vec![]
    }

    fn update(&mut self, item : Cart) -> Option<Cart> {
        None
    }

    fn delete(&mut self , customer_id : String) -> Option<Cart> {
        self.carts.remove(&customer_id)
    }
}


#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn it_adds_a_cart(){  
        
        let mut store = CartStore::new();
        let cart = Cart {
            merchant_id : "one".to_string(),
            ..Cart::default()
        };

        store.add(cart);

        let cart_size  = store.carts.keys().len();
        assert_eq!(1, cart_size )
    }


    #[test]
    fn it_gets_a_cart(){  
        
        let mut store = CartStore::new();
        let cart = Cart {
            merchant_id : "one".to_string(),
            ..Cart::default()
        };
        store.add(cart);

        let cart = store.get("one".to_string()).unwrap();

        assert_eq!("one".to_string(), cart.merchant_id)
    }

    #[test]
    fn it_deletes_a_cart(){  
        
        let mut store = CartStore::new();
        let cart = Cart {
            id : "my_id".to_string(),
            ..Cart::default()
        };
        store.add(cart);
        store.delete("my_id".to_string());

        let cart = store.carts.get("my_id");

        assert_eq!(true, cart.is_none())
    }

}