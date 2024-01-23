use candid::{CandidType, Deserialize};
use std::collections::HashMap;

use crate::cart::*;
use crate::customer::*;
use crate::merchant::*;
use crate::order::*;
use crate::product::*;
use crate::state::Store;

use super::STATE;

// Global state of contract
#[derive(Deserialize, CandidType)]
pub struct Marketplace {
    pub(crate) customers: CustomerStore,
    pub(crate) merchants: MerchantStore,
    pub(crate) orders: OrderStore,
    pub(crate) products: ProductStore,
    pub(crate) carts: CartStore,
}

impl Default for Marketplace {
    fn default() -> Self {
        Self {
            customers: CustomerStore::new(),
            merchants: MerchantStore::new(),
            orders: OrderStore::new(),
            products: ProductStore::new(),
            carts: CartStore::new(),
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
pub fn register_customer(id: String) -> Option<Customer> {
    let mut marketplace = STATE.take();

    // Check if customer exists otherwise create one
    let customer = if let Some(customer) = marketplace.customers.get(id.clone()) {
        // ic_cdk::println!("Customer :{:?} exists", customer);
        Some(customer.clone())
    } else {
        let mut new_customer = Customer::default();
        new_customer.id = id;
        ic_cdk::println!("Created new customer :{:?}", new_customer);
        marketplace.customers.add(new_customer)
    };
    STATE.set(marketplace);
    customer
}

pub fn get_customer(id: String) -> Option<Customer> {
    STATE.take().customers.get(id).cloned()
}

// TODO - add methods for deleting and updating customers

/* ============== Merchant managment ================= */
pub fn register_merchant(id: String) -> Option<Merchant> {
    let mut marketplace = STATE.take();

    let new_merchant = Merchant {
        id: id.clone(),
        ..Merchant::default()
    };

    marketplace.merchants.add(new_merchant.clone());
    marketplace.products.add_merchant(id);

    STATE.set(marketplace);
    Some(new_merchant)
}

/* ============== Product managment ================= */
pub fn add_product(_merchant_id: String, product: Product) -> Option<Product> {
    let new_product = Product {
        sku: product.sku,
        merchant_id: product.merchant_id,
        product_id: product.product_id,
        name: product.name,
        price: product.price,
        description: product.description,
        image_url: product.image_url,
    };

    let principle = ic_cdk::caller();
    ic_cdk::println!("{}", principle);

    let mut marketplace = STATE.take();
    marketplace
        .products
        .add(principle.to_text(), new_product.clone());
    STATE.set(marketplace);
    Some(new_product)
}

pub fn update_product(merchant_id: String, product: Product) -> Option<Product> {
    let mut marketplace = STATE.take();
    marketplace.products.add(merchant_id, product.clone());
    STATE.set(marketplace);
    Some(product)
}

pub fn delete_product(merchant_id: String, sku: String) -> Option<String> {
    let mut marketplace = STATE.take();
    marketplace.products.delete(merchant_id, sku);
    STATE.set(marketplace);
    Some("Deleted".to_string())
}

pub fn get_all_products(merchant_id: String) -> ProductStore {
    let marketplace = STATE.take();
    let products = marketplace.products.get_all(merchant_id);
    STATE.set(marketplace);
    products
}

/* ============== Cart managment ================= */

pub fn add_cart(cart: Cart) -> Option<Cart> {
    let _principle = ic_cdk::caller();
    let mut marketplace = STATE.take();
    marketplace.carts.add(cart.clone());
    STATE.set(marketplace);
    Some(cart)
}

pub fn update_cart(cart: Cart) -> Option<Cart> {
    let mut marketplace = STATE.take();
    marketplace.carts.add(cart.clone());
    STATE.set(marketplace);
    Some(cart)
}

pub fn get_cart(_customer_id: String) -> Option<Cart> {
    let principle = ic_cdk::caller();
    let marketplace = STATE.take();
    marketplace.carts.get(principle.to_text()).cloned()
}

/* ============== Order managment ================= */

pub fn add_order(order: Order) -> Option<Order> {
    let mut marketplace = STATE.take();
    marketplace.orders.add(order.clone());
    STATE.set(marketplace);
    Some(order)
}
