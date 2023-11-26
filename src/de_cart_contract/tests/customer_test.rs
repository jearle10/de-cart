use std::time;
use de_cart_contract::marketplace;

#[test]
fn it_registers_a_customer(){
    let start = time::Instant::now();
    marketplace::register_customer("2vxsx-fae".into());
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
}