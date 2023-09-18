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
#[derive(Debug, Clone, Default, CandidType)]
pub struct ProductStore {
    store: HashMap<String , ProductList>
}

/* Product list structure */
#[derive(Debug, Clone, Default, CandidType)]
pub struct ProductList {
    pub (crate) products: HashMap<String , Product>
}

impl Store<Product, ProductList> for ProductStore {
    fn get(&self, merchant_id: String, id: String) -> Option<Product> {
        let merchant_store = self.store.get(&merchant_id);
        match merchant_store {
            Some(product_list) => {
                product_list.products.get(&id).cloned()
            },
            None => None
        }
    }

    fn add(&mut self, item: Product) -> Option<Product>{

        // Check if merchant exists in store
        let mut merchant_store = self.store.get_mut(&item.clone().merchant_id);

        // Add product to merchant's product list
        match merchant_store {
            Some(product_list) => {
                product_list.products.insert(item.clone().sku, item.clone());
                Some(item.clone())
            },
        // Add merchant to product store and insert new product
            None => {
                self.store.insert(item.clone().merchant_id, Default::default());
                let product_list = self.store.get_mut(&item.clone().merchant_id).unwrap();
                product_list.products.insert(item.clone().sku, item.clone());
                Some(item.clone())
            }
        }
    }

    fn get_all(&self, merchant_id : String) -> Option<ProductList> {
        self.store.get(&merchant_id).cloned()
    }

    fn update(&mut self, item: Product) -> Option<Product> {
        let mut merchant_store = self.store.get_mut(&item.merchant_id);

        match merchant_store {
            Some(product_list) => {

                // Check if product exists and update otherwise return None
                if product_list.clone().products.contains_key(&item.sku ){
                    // Update the existing product
                    product_list.products.insert(item.sku.clone() , item.clone());
                    Some(item)
                } else {
                    None
                }

            },
            None => None
        }
    }

    fn delete(&mut self, merchant_id : String,  id: String) -> Option<Product> {
        let merchant_store = self.store.get_mut(&merchant_id);
        match merchant_store {
            Some(product_list) => product_list.products.remove(&id),
            None => None
        }
    }
}