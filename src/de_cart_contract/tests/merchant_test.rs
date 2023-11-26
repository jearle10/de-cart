use std::time;
use de_cart_contract::marketplace;

#[test]
fn it_registers_a_merchant(){
    let start = time::Instant::now();
    marketplace::register_merchant("2vxsx-fae".into());
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
}