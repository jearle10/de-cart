use crate::state::Store;
use std::collections::HashMap;
use std::io::Error;
use candid::CandidType;
use crate::product;
use crate::product::types::Product;



/* Product store data structure */
#[derive(Debug, CandidType)]
pub struct ProductStore {
    products: HashMap<String , Product>
}

impl Default for ProductStore {
    fn default() -> Self {
        Self {
            products : HashMap::new()
        }
    }
}

impl Store<Product> for ProductStore {
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