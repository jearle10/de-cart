use std::collections::HashMap;
use candid::{ CandidType, Deserialize };

use crate::state::Store;

#[derive(Deserialize, Debug, CandidType, Default, Clone)]
pub struct Customer {
    pub (crate) id : String ,
    pub (crate) email : String ,
    name : String,
    balance : String,
    address: String
}

#[derive(Deserialize, Debug, CandidType, Default, Clone)]
pub struct CustomerStore {
    pub customers : HashMap<String, Customer>
}

impl CustomerStore {
    pub fn new() -> Self {
        Self {
            customers : HashMap::new()
        }
    }
}

impl Store<Customer> for CustomerStore {
    fn get(&self, customer_id : String ) -> Option<&Customer> {
        self.customers.get(&customer_id)
    }

    fn add(&mut self, customer : Customer) -> Option<Customer> {
        // Need to insert based on principal id
        self.customers.insert(customer.id.to_string(), customer.clone());
        Some(customer)
    }

    fn get_all(&self) -> Vec<Customer> {
        self.customers.values().cloned().collect()
    }

    fn update(&mut self, customer : Customer) -> Option<Customer> {
        self.customers.insert(customer.id.to_string(), customer)
    }

    fn delete(&mut self , customer_id : String) -> Option<Customer> {
        self.customers.remove(customer_id.as_str())
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_adds_a_customer(){

        let mut store = CustomerStore::new();

        let new_customer = Customer {
            id : "2vxsx-fae".to_string(),
            ..Customer::default()
        };

        ic_cdk::println!("{:?}", new_customer);
        store.add(new_customer.clone());

        let customers : Vec<&String> = store.customers.keys().collect();
        let length = customers.len();

        assert_eq!(length, 1);
    }

    #[test]
    fn it_deletes_a_customer(){

        let mut store = CustomerStore::new();

        let new_customer = Customer {
            id : "2vxsx-fae".to_string(),
            ..Customer::default()
        };

        ic_cdk::println!("{:?}", new_customer);
        store.add(new_customer.clone());
        store.delete("2vxsx-fae".to_string());

        let customers : Vec<&String> = store.customers.keys().collect();
        let length = customers.len();

        assert_eq!(length, 0);
    }

    #[test]
    fn it_updates_a_customer(){

        let mut store = CustomerStore::new();

        let new_customer = Customer {
            id : "2vxsx-fae".to_string(),
            ..Customer::default()
        };

        ic_cdk::println!("{:?}", new_customer);
        store.add(new_customer.clone());


        let updated_customer = Customer {
            id : "2vxsx-fae".to_string(),
            email : "jian@email.com".to_string(),
            ..Customer::default()
        };

        store.update(updated_customer.clone());


        assert_eq!(store.customers.get("2vxsx-fae").unwrap().email, "jian@email.com".to_string());
    }
}