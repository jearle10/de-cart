

/*
Wrapper struct for the  the shopify api
*/
pub struct Shopify {
}

/*
Implementations for the full shopify api is not needed
Only features needs for app
- Read-only products
- Update Cart
- Checkout

Once the below methods have been tested E2E; they can be
abstracted into an interface / trait and then further platforms can be supported
by implementing this trait ('CommerceProvider' trait)
*/
impl Shopify {
    pub fn api () -> String {
        String::from("Hello from the api")
    }

    pub fn get_product (_id : String){
        todo!()
    }

    pub fn get_all_products(){
        todo!()
    }

    pub fn create_order(_product_id : String){
        todo!()
    }

    pub fn create_fulfilment(_order_id : String){
        todo!()
    }

    pub fn update_fulfilment(_order_id : String){
        todo!()
    }

    pub fn delete_fulfilment(_id : String){
        todo!()
    }
}
