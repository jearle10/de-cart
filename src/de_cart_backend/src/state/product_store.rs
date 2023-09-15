use crate::state::Store;
use std::collections::HashMap;
use std::io::Error;
use candid::{CandidType, Principal};
use crate::product;
use crate::product::types::Product;

/*
Need to update the product store such that products are
grouped by Merchant (use the Principal of merchant to group)

When a call to the canister is made - retrieve the Principle making the call
/ or get some unique identity

New structure:
ProductStore -> HashMap<Principal, ProductList>
ProductList -> HashMap<String, Product>

*/
/* Product store data structure */
#[derive(Debug, CandidType)]
pub struct ProductStore {
    products: HashMap<Principal , ProductList>
}

/* Product list structure */
#[derive(Debug, CandidType)]
pub struct ProductList {
    pub products: HashMap<String , Product>
}

impl Default for ProductList {
    fn default() -> Self {
        Self {
            products : HashMap::new()
        }
    }
}

impl Store<Product> for ProductList {
    fn get(&self, id: String) -> Option<Product> {
        self.products.get(&id).cloned()
    }

    fn add(&mut self, item: Product) {
        self.products.insert(item.sku.to_string(), item);
    }

    fn get_all(&self) -> Result<HashMap<String, Product>, Error> {
        Ok(self.products.clone())
    }

    fn update(&mut self, item: Product) -> Option<Product> {
        if self.products.contains_key(&item.sku ){
            // Update the existing product
            self.products.insert(item.sku.clone() , item.clone());
            Some(item)
        } else {
            None
        }
    }

    fn delete(&mut self, id: String) -> Option<Product> {
        self.products.remove(&id)
    }
}