use candid::{ Deserialize, CandidType };
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct Product {
    pub (crate) sku: String,
    pub (crate) merchant_id : MerchantId,
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

type MerchantId = String;
type ProductId = String;

#[derive(Deserialize, Debug, CandidType, Default, Clone)]
pub struct ProductStore {
    pub (crate) products : HashMap<MerchantId, HashMap<ProductId, Product>>
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

    fn get_merchant_store(&self , merchant_id : String) -> Option<(&ProductId, &HashMap<ProductId, Product>)>{
        self.products.get_key_value(&merchant_id)
    }
}

#[cfg(test)]
mod tests {

    use crate::product;

    use super::*;

    #[test]
    fn it_creates_new_merchant_store(){

        let mut store = ProductStore::new();
        let merchant_id = "one".to_string();
        
        store.add_merchant(merchant_id.clone());
        let merchant_store = store.get_merchant_store(merchant_id.clone()).unwrap();
        assert_eq!(&merchant_id, merchant_store.0);
    }

    #[test]
    fn it_adds_a_product(){

        let mut store = ProductStore::new();
        let merchant_id = "one".to_string();

        let new_product = product::Product {
            merchant_id : merchant_id.clone(),
            sku: "my_sku".to_string(),
            ..Product::default()
        };
        
        store.add_merchant(merchant_id.clone()); // Setup a store for the merchant first
        store.add(merchant_id.clone(), new_product);

        let merchant_store = store.products.get(&merchant_id).cloned().unwrap();
        let product = merchant_store.get("my_sku").cloned().unwrap();
        assert_eq!("my_sku".to_string(), product.sku);
    }


    #[test]
    fn it_gets_a_product(){

        let mut store = ProductStore::new();
        let merchant_id = "one".to_string();

        let new_product = product::Product {
            merchant_id : merchant_id.clone(),
            sku: "my_sku".to_string(),
            ..Product::default()
        };
        
        store.add_merchant(merchant_id.clone()); // Setup a store for the merchant first
        store.add(merchant_id.clone(), new_product);

        let product = store.get(merchant_id, "my_sku".to_string()).unwrap();
        assert_eq!("my_sku".to_string(), product.sku);
    }


    #[test]
    fn it_deletes_a_product(){

        let mut store = ProductStore::new();
        let merchant_id = "one".to_string();

        let new_product = product::Product {
            merchant_id : merchant_id.clone(),
            sku: "my_sku".to_string(),
            ..Product::default()
        };
        
        store.add_merchant(merchant_id.clone()); // Setup a store for the merchant first
        store.add(merchant_id.clone(), new_product);

        let product = store.get(merchant_id.clone(), "my_sku".to_string()).unwrap();

        assert_eq!("my_sku".to_string() , product.sku);

        store.delete(merchant_id.clone(), "my_sku".to_string());

        let product = store.get(merchant_id, "my_sku".to_string());
        assert_eq!(true ,product.is_none());
    }


}