use crate::state::Store;
use std::collections::HashMap;
use std::io::Error;
use candid::CandidType;
use crate::product::types::Product;

/*  Canister state   */
pub struct State {
    // users : Box<dyn Store<T>>,
    // merchants : Box<dyn Store<T>>,
    pub products : Box<dyn Store<Product>>
}

impl Default for State {
    fn default() -> Self {
        State {
            products: Box::new(ProductStore::default()),
        }
    }
}


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

    fn update(&self, id: String, item: Error) {
        ic_cdk::println!("updated product");
    }

    fn delete(&self, id: String) {
        ic_cdk::println!("deleted product");
    }
}