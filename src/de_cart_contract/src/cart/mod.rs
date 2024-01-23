use crate::product;
use crate::state::Store;
use candid::{CandidType, Deserialize};
use std::collections::HashMap;

type CustomerId = String;

#[derive(Deserialize, CandidType, Default, Clone)]
pub struct Cart {
    id: String,
    user_id: String,
    merchant_id: String,
    items: Vec<CartItem>,
}

impl Cart {
    fn total_bytes(&self) -> usize {
        let mut bytes = 0;
        bytes += self.id.len();
        bytes += self.user_id.len();
        bytes += self.merchant_id.len();
        self.items
            .iter()
            .for_each(|item| bytes += item.product_id.len() + u32::BITS as usize);
        bytes
    }
}

#[derive(Deserialize, CandidType, Default, Clone)]
pub(crate) struct CartItem {
    pub(crate) product_id: String,
    pub(crate) qty: u32,
}
#[derive(Deserialize, CandidType, Default, Clone)]
pub struct CartStore {
    pub(crate) carts: HashMap<String, Cart>,
}

impl CartStore {
    pub fn new() -> Self {
        Self {
            carts: HashMap::new(),
        }
    }
}

impl Store<Cart> for CartStore {
    fn get(&self, customer_id: String) -> Option<&Cart> {
        self.carts.get(&customer_id)
    }

    fn add(&mut self, cart: Cart) -> Option<Cart> {
        self.carts.insert(cart.id.clone(), cart)
    }

    fn get_all(&self) -> Vec<Cart> {
        vec![]
    }

    fn update(&mut self, cart: Cart) -> Option<Cart> {
        self.add(cart)
    }

    fn delete(&mut self, customer_id: String) -> Option<Cart> {
        self.carts.remove(&customer_id)
    }
}

impl CartStore {
    pub(crate) fn total_bytes(&self) -> usize {
        let mut total_bytes = 0;
        self.carts
            .iter()
            .for_each(|(id, cart)| total_bytes += id.len() + cart.total_bytes());
        total_bytes
    }

    pub(crate) fn total_carts(&self) -> String {
        self.carts.iter().len().to_string()
    }
}

#[cfg(test)]
mod tests {

    use crate::cart;

    use super::*;
    // extern crate test;
    // use test::Bencher;

    #[test]
    fn it_adds_a_cart() {
        let mut store = CartStore::new();
        let cart = Cart {
            id: "one".to_string(),
            ..Cart::default()
        };

        store.add(cart);

        let cart_size = store.carts.keys().len();
        assert_eq!(1, cart_size)
    }

    #[test]
    fn it_gets_a_cart() {
        let mut store = CartStore::new();
        let cart = Cart {
            id: "one".to_string(),
            ..Cart::default()
        };
        store.add(cart);

        let cart = store.get("one".to_string()).unwrap();

        assert_eq!("one".to_string(), cart.id)
    }

    #[test]
    fn it_deletes_a_cart() {
        let mut store = CartStore::new();
        let cart = Cart {
            id: "my_id".to_string(),
            ..Cart::default()
        };
        store.add(cart);
        store.delete("my_id".to_string());

        let cart = store.carts.get("my_id");

        assert!(cart.is_none())
    }

    #[test]
    fn it_calculates_bytes_of_cart() {
        let cart1 = Cart {
            id: "1".to_string(),
            user_id: "1".to_string(),
            merchant_id: "1".to_string(),
            items: vec![
                CartItem {
                    product_id: "1".to_string(),
                    qty: 4,
                },
                CartItem {
                    product_id: "2".to_string(),
                    qty: 4,
                },
            ],
        };

        let cart_storage = cart1.total_bytes();

        assert_eq!(cart_storage, 69)
    }

    #[test]
    fn it_calculates_bytes_of_cart_store() {
        let cart1 = Cart {
            id: "1".to_string(),
            user_id: "1".to_string(),
            merchant_id: "1".to_string(),
            items: vec![
                CartItem {
                    product_id: "1".to_string(),
                    qty: 4,
                },
                CartItem {
                    product_id: "2".to_string(),
                    qty: 4,
                },
            ],
        };

        let cart2 = Cart {
            id: "2".to_string(),
            user_id: "2".to_string(),
            merchant_id: "2".to_string(),
            items: vec![
                CartItem {
                    product_id: "1".to_string(),
                    qty: 4,
                },
                CartItem {
                    product_id: "2".to_string(),
                    qty: 4,
                },
            ],
        };

        let mut store = cart::CartStore::default();

        store.add(cart1);
        store.add(cart2);

        let cart_store_storage = store.total_bytes();

        assert_eq!(cart_store_storage, 140)
    }
}
