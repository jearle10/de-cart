pub (crate) mod types;

use ic_cdk::export::candid::{ candid_method, Deserialize, CandidType };
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader,
    HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};

use base64;
use base64::Engine;
use ic_cdk::api::call::CallResult;
use types::Product;

#[ic_cdk::update]
#[candid_method(update)]
fn list_product() -> String {
    format!("Listed product on de-cart")
}

#[ic_cdk::update]
#[candid_method(update)]
fn delist_product() -> String {
    format!("Removed product on de-cart")
}

#[ic_cdk::query]
#[candid_method(query)]
fn list_products() -> String {
    format!("Listed products")
}

#[ic_cdk::query]
#[candid_method(update)]
fn get_product() -> String {
    format!("Retrieved product")
}

#[ic_cdk::update]
#[candid_method(update)]
async fn import_products(
    username : String,
    password : String,
    host : String,
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

    match http_request(request).await {
        Ok((response,)) => {
            let body = String::from_utf8(response.body)
                .expect("Invalid utf8 string received");
            ic_cdk::println!("{:?}", body)
        },
        Err(e) => ic_cdk::println!("{:?}", e),
    }

    // ic_cdk::println!("Testing printing logs");
    format!("Fetched cscart products")
}