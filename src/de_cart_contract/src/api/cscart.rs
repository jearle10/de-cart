use ic_cdk::api::management_canister::http_request::{
    HttpMethod,
    HttpHeader,
    HttpResponse,
    CanisterHttpRequestArgument,
    http_request
};

use base64;
use base64::Engine;

/*
Wrapper struct for the  the shopify api
*/
pub struct Cscart {
}

impl Cscart {
    pub fn api () -> String {
        String::from("Hello from the api")
    }

    pub fn get_product () -> String {
        todo!()
    }
    pub async fn get_all_products(
        username : String ,
        password : String,
        host : String
    ) -> String {
        // Construct basic auth token using credentials
        let credentials = format!("{}:{}", username , password);
        let engine = base64::engine::general_purpose::STANDARD;
        let token = engine.encode(credentials.as_bytes());

        let url = format!("https://{}/api/2.0/products", host);

        let request_headers = vec![
            HttpHeader {
                name: "Host".to_string(),
                value: format!("{host}:443"),
            },
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {token}"),
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

        match http_request(request, 1_603_129_200).await {
            Ok((response,)) => {
                let body = String::from_utf8(response.body)
                    .expect("Invalid utf8 string received");
                ic_cdk::println!("{:?}", body)
            },
            Err(e) => ic_cdk::println!("{:?}", e),
        }

        ic_cdk::println!("Testing printing logs");
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
