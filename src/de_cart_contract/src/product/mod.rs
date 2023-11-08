use candid::{ Deserialize, CandidType };
use std::collections::HashMap;
#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct Product {
    pub (crate) sku: String,
    pub (crate) merchant_id : String,
    pub (crate) product_id : String,
    pub (crate) name: String,
    pub (crate) price: candid::Nat,
    pub (crate) description: String,
    pub (crate) image_url: String
}

impl Default for Product {
    fn default() -> Self {
        Product {
            sku: "default".to_string(),
            product_id: "default_Product".to_string(),
            name: "".to_string(),
            price: Default::default(),
            description: "".to_string(),
            image_url: "".to_string(),
            merchant_id: "".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, CandidType, Default, Clone)]
pub struct ProductStore {
    pub (crate) products : HashMap<String, HashMap<String, Product>>
}


impl ProductStore {
    pub fn new() -> Self {
        Self {
            products : HashMap::new()
        }
    }
}


impl ProductStore {
    pub fn get(&self, merchant_id : String , sku : String) -> Option<Product> {
        let products = self.products.get(&merchant_id).unwrap().clone();
        products.get(&sku).cloned()
    }

    pub fn add(&mut self, merchant_id : String , product : Product) -> Option<Product> {
        let merchant_products = self.products.get_mut(&merchant_id).unwrap();
        merchant_products.insert(product.sku.to_string(), product.clone());
        Some(product)
    }

    pub fn get_all(&self, merchant_id : String) -> ProductStore {
        // let products : Vec<Product> = merchant_products.values().cloned().collect();
        // products
        self.clone()
    }

    pub fn update(&mut self,merchant_id : String,  product : Product) -> Option<Product> {
        let merchant_products = self.products.get_mut(&merchant_id).unwrap();
        merchant_products.insert(product.sku.to_string(), product.clone());
        Some(product)
    }

    pub fn delete(&mut self, merchant_id : String, sku : String) -> Option<Product> {
        let merchant_products = self.products.get_mut(&merchant_id).unwrap();
        merchant_products.remove(&sku)
    }

    pub fn add_merchant(&mut self , merchant_id : String){
        self.products.insert(merchant_id.to_string(), HashMap::new());
    }
}
