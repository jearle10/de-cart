#![allow(unused_imports)]
// #![feature(test)]

mod customer;
mod merchant;
pub mod marketplace;
mod order;
mod product;
mod cart;
mod state;
// mod api;

use candid::{CandidType, Deserialize};
use ic_cdk::api::stable::WASM_PAGE_SIZE_IN_BYTES;
use marketplace::Marketplace;
use customer::*;
use merchant::*;
use order::Order;
use product::*;
use cart::*;
use std::cell::RefCell;
use ic_cdk;
use std::mem;
use ic_cdk::export_candid;


use ic_cdk::update;
use std::str::FromStr;
use types::{
    CanisterId, VetKDCurve, VetKDEncryptedKeyReply, VetKDEncryptedKeyRequest, VetKDKeyId,
    VetKDPublicKeyReply, VetKDPublicKeyRequest,
};

mod types;

const VETKD_SYSTEM_API_CANISTER_ID: &str = "s55qq-oqaaa-aaaaa-aaakq-cai";

/*
For now the users will remain anon. Each cart will have an associated
principle id /wallet that has permission to manipulate items in it via
access control
 */
thread_local! {
    static STATE : RefCell<Marketplace> = RefCell::new(Marketplace::default())


    // 96GiB of Stable memory in a canister currently
    // 4GiB heap size
    // Define some sensible cansiter limits to ensure upgrades possible
}

#[derive(Deserialize, CandidType)]
struct MarketplaceStats {
    total : String,
    customers : String,
    merchants : String,
    products : String,
    carts : String,
    orders : String
}

#[ic_cdk::update]
fn register_customer()-> Option<Customer> {
    let id = ic_cdk::api::caller().to_text();
    marketplace::register_customer(id)
}

#[ic_cdk::query]
fn get_customer() -> Option<Customer>{
    let id = ic_cdk::api::caller().to_text();
    marketplace::get_customer(id)
}


#[ic_cdk::update]
fn register_merchant() -> Option<Merchant> {
    let principle = ic_cdk::caller().to_text();
    marketplace::register_merchant(principle)
}


#[ic_cdk::update]
fn add_product(product : Product) -> Option<Product> {
    let principle = ic_cdk::caller().to_text();
    marketplace::add_product(principle, product)
}

#[ic_cdk::update]
fn update_product(product : Product) -> Option<Product> {
    let merchant_id = ic_cdk::caller().to_text();
    marketplace::update_product(merchant_id, product)
}


#[ic_cdk::update]
fn delete_product(sku : String) -> Option<String> {
    let merchant_id = ic_cdk::caller().to_text();
    marketplace::delete_product(merchant_id, sku)
}


#[ic_cdk::update]
fn get_all_products() -> ProductStore {
    let merchant_id = ic_cdk::caller().to_text();
    ic_cdk::println!("{}", merchant_id);
    marketplace::get_all_products(merchant_id)
}

#[ic_cdk::update]
fn add_cart(cart : Cart) -> Option<Cart> {
    let merchant_id =  ic_cdk::caller().to_text();
    marketplace::add_cart(cart)
}

#[ic_cdk::update]
fn update_cart(cart : Cart) -> Option<Cart> {
    let merchant_id =  ic_cdk::caller().to_text();
    marketplace::add_cart(cart)
}

#[ic_cdk::update]
fn get_cart() -> Option<Cart> {
    let customer_id =  ic_cdk::caller().to_text();
    marketplace::get_cart(customer_id)
}

#[ic_cdk::update]
fn add_order(order : Order) -> Option<Order> {
    let customer_id =  ic_cdk::caller().to_text();
    marketplace::add_order(order)
}


#[ic_cdk::query]
fn test()-> String {
    String::from("Hello")
}

#[ic_cdk::query]
fn marketplace_stats() -> MarketplaceStats {
    ic_cdk::println!("{}", ic_cdk::caller().to_text());

    let marketplace = STATE.take();

    let mut bytes = 0;

    let customers_size =  marketplace.customers.total_bytes();
    let merchants_size = marketplace.merchants.total_bytes();
    let products_size = marketplace.products.total_bytes();
    let carts_size = marketplace.carts.total_bytes();
    let orders_size =  marketplace.orders.total_bytes();

    let customers_count =  marketplace.customers.total_customer();
    let merchants_count = marketplace.merchants.total_merchants();
    let products_count = marketplace.products.total_products();
    let carts_count = marketplace.carts.total_carts();
    let orders_count =  marketplace.orders.total_orders();

    bytes += customers_size + merchants_size + products_size + carts_size + orders_size;

    let count = marketplace.products.total_products();
    STATE.set(marketplace);

    let stats = MarketplaceStats { 
        total: format!("bytes: {}", bytes), 
        customers: format!("count: {}, size: {}", customers_count, customers_size),  
        merchants: format!("count: {}, size: {} ", merchants_count, merchants_size),  
        products: format!("count: {}, size: {}", products_count, products_size), 
        carts: format!("count: {}, size: {}", carts_count, carts_size),
        orders: format!("count: {}, size: {}", orders_count, orders_size)  
    };
    stats
}

#[update]
async fn symmetric_key_verification_key() -> String {
    let request = VetKDPublicKeyRequest {
        canister_id: None,
        derivation_path: vec![b"symmetric_key".to_vec()],
        key_id: bls12_381_test_key_1(),
    };

    let (response,): (VetKDPublicKeyReply,) = ic_cdk::api::call::call(
        vetkd_system_api_canister_id(),
        "vetkd_public_key",
        (request,),
    )
    .await
    .expect("call to vetkd_public_key failed");

    hex::encode(response.public_key)
}

#[update]
async fn encrypted_symmetric_key_for_caller(encryption_public_key: Vec<u8>) -> String {
    debug_println_caller("encrypted_symmetric_key_for_caller");

    let request = VetKDEncryptedKeyRequest {
        derivation_id: ic_cdk::caller().as_slice().to_vec(),
        public_key_derivation_path: vec![b"symmetric_key".to_vec()],
        key_id: bls12_381_test_key_1(),
        encryption_public_key,
    };

    let (response,): (VetKDEncryptedKeyReply,) = ic_cdk::api::call::call(
        vetkd_system_api_canister_id(),
        "vetkd_encrypted_key",
        (request,),
    )
    .await
    .expect("call to vetkd_encrypted_key failed");

    hex::encode(response.encrypted_key)
}

#[update]
async fn ibe_encryption_key() -> String {
    let request = VetKDPublicKeyRequest {
        canister_id: None,
        derivation_path: vec![b"ibe_encryption".to_vec()],
        key_id: bls12_381_test_key_1(),
    };

    let (response,): (VetKDPublicKeyReply,) = ic_cdk::api::call::call(
        vetkd_system_api_canister_id(),
        "vetkd_public_key",
        (request,),
    )
    .await
    .expect("call to vetkd_public_key failed");

    hex::encode(response.public_key)
}

#[update]
async fn encrypted_ibe_decryption_key_for_caller(encryption_public_key: Vec<u8>) -> String {
    debug_println_caller("encrypted_ibe_decryption_key_for_caller");

    let request = VetKDEncryptedKeyRequest {
        derivation_id: ic_cdk::caller().as_slice().to_vec(),
        public_key_derivation_path: vec![b"ibe_encryption".to_vec()],
        key_id: bls12_381_test_key_1(),
        encryption_public_key,
    };

    let (response,): (VetKDEncryptedKeyReply,) = ic_cdk::api::call::call(
        vetkd_system_api_canister_id(),
        "vetkd_encrypted_key",
        (request,),
    )
    .await
    .expect("call to vetkd_encrypted_key failed");

    hex::encode(response.encrypted_key)
}

fn bls12_381_test_key_1() -> VetKDKeyId {
    VetKDKeyId {
        curve: VetKDCurve::Bls12_381,
        name: "test_key_1".to_string(),
    }
}

fn vetkd_system_api_canister_id() -> CanisterId {
    CanisterId::from_str(VETKD_SYSTEM_API_CANISTER_ID).expect("failed to create canister ID")
}

fn debug_println_caller(method_name: &str) {
    ic_cdk::println!(
        "{}: caller: {} (isAnonymous: {})",
        method_name,
        ic_cdk::caller().to_text(),
        ic_cdk::caller() == candid::Principal::anonymous()
    );
}




export_candid!();