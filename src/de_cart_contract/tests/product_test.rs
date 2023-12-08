// use ic_agent::{Agent, export::Principal};
// use candid::{Encode, Decode, CandidType, Nat};


// #[test]
// fn e2e_get_all_product(){

//     let URL = String::from("http://127.0.0.1:4943");

//     let agent = Agent::builder()
//         .with_url(URL)
//         .with_identity(create_identity())
//         .build()?;

//     agent.fetch_root_key().await?;

//     let effective_canister_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
//     let response = agent.query("bkyz2-fmaaa-aaaaa-qaaaq-cai", "get_all_products")
//         .with_effective_canister_id(effective_canister_id)
//         .call_and_wait()
//         .await?;


//     let result = Decode!(response.as_slice(), CreateCanisterResult)?;
//     println!("{}",result)

//     }