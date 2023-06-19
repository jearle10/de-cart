#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
/* Should probably have separate canisters for
merchant functions and user TBC on design
 */

// User functions
#[ic_cdk::query]
fn add_to_cart() -> String {
    format!("Added product to cart")
}

#[ic_cdk::query]
fn delete_from_cart() -> String {
    format!("Remove product from cart")
}

#[ic_cdk::query]
fn add_product() -> String {
    format!("Remove product from cart")
}

// Merchant functions
#[ic_cdk::query]
fn remove_product() -> String {
    format!("Listed product on de-cart")
}

#[ic_cdk::query]
fn import_products() -> String {
    format!("Fetched products from e-commerce platform")
}