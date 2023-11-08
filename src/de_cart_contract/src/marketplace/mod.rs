use candid::{ Deserialize, CandidType };
use std::collections::HashMap;

use crate::customer;
use crate::customer::Customer;
use crate::product::{self , Product};
use crate::merchant;
use crate::order;
use crate::cart;
use crate::state::Store;

use super::STATE;

#[derive(Deserialize, CandidType)]
pub struct Marketplace {
    customers : customer::CustomerStore,
    merchants : merchant::MerchantStore,
    orders : HashMap<String, order::Order>,
    pub (crate) products: product::ProductStore,
    carts : HashMap<String , cart::Cart>
}

#[ic_cdk::query]
fn get_customer(id : String) -> Option<customer::Customer>{
    STATE.take().customers.get(id).cloned()
}

#[ic_cdk::update]
fn register_customer() -> Option<customer::Customer> {
    
    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);

    let mut marketplace = STATE.take();

    // Check if customer exists otherwise create one
    let customer = if let Some(customer) = marketplace.customers.get(principle.to_text()) {
        ic_cdk::println!("Customer :{:?} exists", customer);
        Some(customer.clone())
    } else {
        let mut new_customer = Customer::default();
        new_customer.id = principle.to_text();
        ic_cdk::println!("Created new customer :{:?}", new_customer);
        marketplace.customers.add(new_customer)
    };
    STATE.set(marketplace);
    customer
}


#[ic_cdk::update]
fn register_merchant() -> Option<merchant::Merchant> {
    let mut marketplace = STATE.take();

    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);

    let new_merchant = merchant::Merchant {
        id : principle.to_text(),
        ..merchant::Merchant::default()
    };

    marketplace.merchants.add(new_merchant.clone());
    marketplace.products.add_merchant(principle.to_text());

    STATE.set(marketplace);
    Some(new_merchant)
}


#[ic_cdk::update]
fn add_product(sku : String) -> Option<product::Product>{
    let new_product = product::Product {
        sku : sku,
        ..product::Product::default()
    };

    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);
    
    let mut marketplace = STATE.take();
    marketplace.products.add(principle.to_text(), new_product.clone());
    STATE.set(marketplace);
    Some(new_product)
}

#[ic_cdk::update]
fn update_product(merchant_id :String, product: Product) -> Option<product::Product>{

    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);
    
    let mut marketplace = STATE.take();
    marketplace.products.add(principle.to_text(), product.clone());
    STATE.set(marketplace);
    Some(product)
}

#[ic_cdk::update]
fn delete_product(merchant_id :String, sku : String) -> Option<String>{

    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);
    
    let mut marketplace = STATE.take();
    marketplace.products.delete(principle.to_text(), sku);
    STATE.set(marketplace);
    Some("Deleted".to_string())
}

#[ic_cdk::update]
fn get_all_products() -> product::ProductStore{
    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);
    
    let marketplace = STATE.take();
    let products = marketplace.products.get_all(principle.to_text());
    STATE.set(marketplace);
    products
}







impl Default for Marketplace {
    fn default() -> Self {
        Self {
            customers : customer::CustomerStore::new(),
            merchants: merchant::MerchantStore::new(),
            orders: HashMap::new(),
            products: product::ProductStore::new(),
            carts: HashMap::new(),
        }
    }
}