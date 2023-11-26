use std::time;
use de_cart_contract::marketplace;

#[test]
fn it_registers_a_customer(){
    println!("Hello from e2e");
    marketplace::register_customer("2vxsx-fae");
    let start = time::Instant::now();
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
}