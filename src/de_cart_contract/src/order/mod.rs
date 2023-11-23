use std::collections::HashMap;

use candid::{ Deserialize , CandidType};

use crate::state::Store;
use crate::cart::CartItem;

// use crate::product::P

#[derive(Deserialize, CandidType, Clone, Default)]
pub struct Order {
    id : String , 
    status : String , 
    merchant_id : String ,
    customer_id : String ,
    amount : String ,
    tracking_number : String,
    carrier : String ,
    items: Vec<CartItem>, // Todo update to Product struct,
    address : String
}


impl Order {

    fn total_bytes(&self) -> usize {
        let mut bytes = 0;
        bytes += self.id.len();
        bytes += self.status.len();
        bytes += self.merchant_id.len();
        bytes += self.customer_id.len();
        bytes += self.amount.len();
        bytes += self.tracking_number.len();
        bytes += self.carrier.len();
        self.items.iter().for_each(|item | bytes += item.product_id.len() + u32::BITS as usize);
        bytes += self.address.len();
        bytes
    }
}

#[derive(Deserialize, CandidType, Clone, Default)]
pub struct OrderStore {
    orders : HashMap<String , Order>
}

impl OrderStore {
    pub fn new() -> Self {
        Self {
            orders : HashMap::new()
        }
    }
}

impl Store<Order> for OrderStore {
    fn get(&self, id : String ) -> Option<&Order> {
        self.orders.get(&id)
    }

    fn add(&mut self, order : Order) -> Option<Order> {
        // Need to insert based on principal id
        self.orders.insert(order.id.to_string(), order.clone());
        Some(order)
    }

    fn get_all(&self) -> Vec<Order> {
        self.orders.values().cloned().collect()
    }

    fn update(&mut self, order : Order) -> Option<Order> {
        self.orders.insert(order.id.to_string(), order)
    }

    fn delete(&mut self , id : String) -> Option<Order> {
        self.orders.remove(&id)
    }
}


impl OrderStore {
    pub (crate) fn total_bytes(&self) -> usize {
        let mut total_bytes = 0;
        self.orders
            .iter()
            .for_each(|(id , order)| total_bytes += id.len() + order.total_bytes());
        total_bytes
    }

    pub (crate) fn total_orders(&self) -> String {
        self.orders.iter().len().to_string()
    }
}





#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn it_calculates_bytes_of_order(){
        
        let order = Order { 
            id: "1".to_string(), 
            status: "1".to_string(),
            merchant_id: "1".to_string(),
            customer_id: "1".to_string(),
            amount: "1".to_string(),
            tracking_number: "1".to_string(),
            carrier: "1".to_string(),
            items: vec![
                    CartItem { product_id: "1".to_string(), qty: 4 },
                    CartItem { product_id: "2".to_string(), qty: 4 }
                    ],
            address: "1".to_string(),
      
        };
        
        let order_storage = order.total_bytes();
        assert_eq!(order_storage , 74)
    }

    #[test]
    fn it_calculates_bytes_of_order_store(){

        let order1 = Order { 
            id: "1".to_string(), 
            status: "1".to_string(),
            merchant_id: "1".to_string(),
            customer_id: "1".to_string(),
            amount: "1".to_string(),
            tracking_number: "1".to_string(),
            carrier: "1".to_string(),
            items: vec![
                    CartItem { product_id: "1".to_string(), qty: 4 },
                    CartItem { product_id: "2".to_string(), qty: 4 }
                    ],
            address: "1".to_string(),
      
        };

        let order2 = Order { 
            id: "2".to_string(), 
            status: "2".to_string(),
            merchant_id: "2".to_string(),
            customer_id: "2".to_string(),
            amount: "2".to_string(),
            tracking_number: "2".to_string(),
            carrier: "2".to_string(),
            items: vec![
                    CartItem { product_id: "1".to_string(), qty: 4 },
                    CartItem { product_id: "2".to_string(), qty: 4 }
                    ],
            address: "2".to_string(),
      
        };
        
    
        let mut store = OrderStore::default();

        store.add(order1);
        store.add(order2);

        let order_store_storage = store.total_bytes();
        assert_eq!(order_store_storage , 150)
    }

}