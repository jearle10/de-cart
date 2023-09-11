use ic_cdk::export::candid::{ candid_method, Deserialize, CandidType };
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader,
    HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};

use base64;
use base64::Engine;
use ic_cdk::api::call::CallResult;

#[derive(Clone, Debug, Default, Deserialize, CandidType)]
pub struct Product {
    sku: String,
    name: String,
    price: candid::Nat,
    image_url: String
}
