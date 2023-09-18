use ic_cdk::api::management_canister::http_request::{
    HttpMethod,
    HttpHeader,
    HttpResponse,
    CanisterHttpRequestArgument,
    http_request
};

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

    pub fn get_product () -> String {
        todo!()
    }

    pub async fn get_all_products(access_token : String, host : String) -> String {

        let url = format!(
            "https://{}/admin/api/2023-07/products.json?ids&fields=id,title,status",
            host
        );
        let request_headers = vec![
            // HttpHeader {
            //     name: "Host".to_string(),
            //     value: format!("{host}:443"),
            // },
            HttpHeader {
                name: "X-Shopify-Access-Token".to_string(),
                value: access_token,
            },
        ];

        let request = CanisterHttpRequestArgument {
            url,
            max_response_bytes: None, //optional for request
            method: HttpMethod::GET,
            headers: request_headers,
            body: None,      //optional for request
            transform: None, //optional for request
        };

        // 1_603_129_200 cycles - seems to be the min required from testing locally
        // This will need increasing when deployed to a public subnet

        match http_request(request, 1_603_129_200).await {
            Ok((response,)) => {
                let body = String::from_utf8(response.body)
                    .expect("Invalid utf8 string received");
                ic_cdk::println!("{:?}", body)
            },
            Err(e) => ic_cdk::println!("{:?}", e),
        }
        format!("Fetched cscart products")
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
