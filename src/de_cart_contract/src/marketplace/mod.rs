use candid::{ Deserialize, CandidType };
use std::collections::HashMap;

use crate::customer::*;
use crate::product::*;
use crate::merchant::*;
use crate::order::*;
use crate::cart::*;
use crate::state::Store;

use super::STATE;

// Global state of contract 
#[derive(Deserialize, CandidType)]
pub struct Marketplace {
    pub (crate) customers : CustomerStore,
    pub (crate) merchants : MerchantStore,
    pub (crate) orders : OrderStore,
    pub (crate) products: ProductStore,
    pub (crate) carts : CartStore
}

impl Default for Marketplace {
    fn default() -> Self {
        Self {
            customers : CustomerStore::new(),
            merchants: MerchantStore::new(),
            orders: OrderStore::new(),
            products: ProductStore::new(),
            carts: CartStore::new()
        }
    }
}

/* 
All the services provided by the marketplace
- Merchants 
- Customers
- Orders
- Baskets 
*/

/* ============== Customer managment ================= */ 

#[ic_cdk::update]
fn register_customer() -> Option<Customer> {
    
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


#[ic_cdk::query]
fn get_customer(id : String) -> Option<Customer>{
    STATE.take().customers.get(id).cloned()
}

// TODO - add methods for deleting and updating customers


/* ============== Merchant managment ================= */ 

#[ic_cdk::update]
fn register_merchant() -> Option<Merchant> {
    let mut marketplace = STATE.take();

    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);

    let new_merchant = Merchant {
        id : principle.to_text(),
        ..Merchant::default()
    };

    marketplace.merchants.add(new_merchant.clone());
    marketplace.products.add_merchant(principle.to_text());

    STATE.set(marketplace);
    Some(new_merchant)
}


/* ============== Product managment ================= */
#[ic_cdk::update]
fn add_product(product : Product) -> Option<Product>{
    let new_product = Product { 
        sku: product.sku, 
        merchant_id: product.merchant_id, 
        product_id: product.product_id, 
        name: product.name, 
        price: product.price, 
        description: product.description, 
        image_url: product.image_url 
    };

    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);
    
    let mut marketplace = STATE.take();
    marketplace.products.add(principle.to_text(), new_product.clone());
    STATE.set(marketplace);
    Some(new_product)
}

#[ic_cdk::update]
fn update_product(_merchant_id :String, product: Product) -> Option<Product>{

    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);
    
    let mut marketplace = STATE.take();
    marketplace.products.add(principle.to_text(), product.clone());
    STATE.set(marketplace);
    Some(product)
}

#[ic_cdk::update]
fn delete_product(_merchant_id :String, sku : String) -> Option<String>{

    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);
    
    let mut marketplace = STATE.take();
    marketplace.products.delete(principle.to_text(), sku);
    STATE.set(marketplace);
    Some("Deleted".to_string())
}

#[ic_cdk::update]
fn get_all_products() -> ProductStore{
    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);
    
    let marketplace = STATE.take();
    let products = marketplace.products.get_all(principle.to_text());
    STATE.set(marketplace);
    products
}


/* ============== Cart managment ================= */


#[ic_cdk::update]
fn add_cart(cart : Cart) -> Option<Cart> {
    let _principle =  ic_cdk::caller();
    let mut marketplace = STATE.take();
    marketplace.carts.add(cart.clone());
    STATE.set(marketplace);
    Some(cart)
}


#[ic_cdk::update]
fn update_cart(cart : Cart) -> Option<Cart> {
    let _principle =  ic_cdk::caller();
    let mut marketplace = STATE.take();
    marketplace.carts.add(cart.clone());
    STATE.set(marketplace);
    Some(cart)
}

#[ic_cdk::update]
fn get_cart() -> Option<Cart> {
    let principle =  ic_cdk::caller();
    let marketplace = STATE.take();
    marketplace.carts.get(principle.to_text()).cloned()
}


/* ============== Order managment ================= */

#[ic_cdk::update]
fn add_order(order : Order) -> Option<Order> {
    let _principle =  ic_cdk::caller();
    let mut marketplace = STATE.take();
    marketplace.orders.add(order.clone());
    STATE.set(marketplace);
    Some(order)
}