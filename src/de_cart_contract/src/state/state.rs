use crate::state::{ Store , ProductStore};
use crate::product::types::Product;
use crate::state::ProductList;

/*  Canister state   */
pub struct State {
    // users : Box<dyn Store<T>>,
    // merchants : Box<dyn Store<T>>,
    pub products : Box<dyn Store<Product, ProductList>>
}

impl Default for State {
    fn default() -> Self {
        State {
            products: Box::new(ProductStore::default()),
        }
    }
}