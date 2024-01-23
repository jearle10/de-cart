use candid::{CandidType, Deserialize};
use std::collections::HashMap;

use crate::state::Store;

#[derive(Deserialize, Debug, CandidType, Default, Clone)]
pub struct Customer {
    pub id: String,
    pub(crate) email: String,
    name: String,
    balance: String,
    address: String,
}

impl Customer {
    fn total_bytes(&self) -> usize {
        let mut bytes = 0;
        bytes += self.id.len();
        bytes += self.email.len();
        bytes += self.name.len();
        bytes += self.balance.len();
        bytes += self.address.len();
        bytes
    }
}

#[derive(Deserialize, Debug, CandidType, Default, Clone)]
pub struct CustomerStore {
    pub customers: HashMap<String, Customer>,
}

impl CustomerStore {
    pub fn new() -> Self {
        Self {
            customers: HashMap::new(),
        }
    }
}

impl Store<Customer> for CustomerStore {
    fn get(&self, customer_id: String) -> Option<&Customer> {
        self.customers.get(&customer_id)
    }

    fn add(&mut self, customer: Customer) -> Option<Customer> {
        // Need to insert based on principal id
        self.customers
            .insert(customer.id.to_string(), customer.clone());
        Some(customer)
    }

    fn get_all(&self) -> Vec<Customer> {
        self.customers.values().cloned().collect()
    }

    fn update(&mut self, customer: Customer) -> Option<Customer> {
        self.customers.insert(customer.id.to_string(), customer)
    }

    fn delete(&mut self, customer_id: String) -> Option<Customer> {
        self.customers.remove(customer_id.as_str())
    }
}

impl CustomerStore {
    pub(crate) fn total_bytes(&self) -> usize {
        let mut total_bytes = 0;
        self.customers
            .iter()
            .for_each(|(id, customer)| total_bytes += id.len() + customer.total_bytes());
        total_bytes
    }

    pub(crate) fn total_customer(&self) -> String {
        self.customers.iter().len().to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_adds_a_customer() {
        let mut store = CustomerStore::new();

        let new_customer = Customer {
            id: "2vxsx-fae".to_string(),
            ..Customer::default()
        };

        ic_cdk::println!("{:?}", new_customer);
        store.add(new_customer.clone());

        let customers: Vec<&String> = store.customers.keys().collect();
        let length = customers.len();

        assert_eq!(length, 1);
    }

    #[test]
    fn it_deletes_a_customer() {
        let mut store = CustomerStore::new();

        let new_customer = Customer {
            id: "2vxsx-fae".to_string(),
            ..Customer::default()
        };

        ic_cdk::println!("{:?}", new_customer);
        store.add(new_customer.clone());
        store.delete("2vxsx-fae".to_string());

        let customers: Vec<&String> = store.customers.keys().collect();
        let length = customers.len();

        assert_eq!(length, 0);
    }

    #[test]
    fn it_updates_a_customer() {
        let mut store = CustomerStore::new();

        let new_customer = Customer {
            id: "2vxsx-fae".to_string(),
            ..Customer::default()
        };

        ic_cdk::println!("{:?}", new_customer);
        store.add(new_customer.clone());

        let updated_customer = Customer {
            id: "2vxsx-fae".to_string(),
            email: "jian@email.com".to_string(),
            ..Customer::default()
        };

        store.update(updated_customer.clone());

        assert_eq!(
            store.customers.get("2vxsx-fae").unwrap().email,
            "jian@email.com".to_string()
        );
    }

    #[test]
    fn it_calculates_bytes_of_customer() {
        let customer = Customer {
            id: "1".to_string(),
            email: "1".to_string(),
            name: "1".to_string(),
            balance: "1".to_string(),
            address: "1".to_string(),
        };

        let customer_storage = customer.total_bytes();
        assert_eq!(customer_storage, 5)
    }

    #[test]
    fn it_calculates_bytes_of_customer_store() {
        let customer1 = Customer {
            id: "1".to_string(),
            email: "1".to_string(),
            name: "1".to_string(),
            balance: "1".to_string(),
            address: "1".to_string(),
        };

        let customer2 = Customer {
            id: "2".to_string(),
            email: "2".to_string(),
            name: "2".to_string(),
            balance: "2".to_string(),
            address: "2".to_string(),
        };

        let mut store = CustomerStore::default();

        store.add(customer1);
        store.add(customer2);

        let customer_store_storage = store.total_bytes();

        assert_eq!(customer_store_storage, 12)
    }
}
