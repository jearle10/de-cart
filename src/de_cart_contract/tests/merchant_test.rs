use de_cart_contract::marketplace;
use std::time;

#[test]
fn it_registers_a_merchant() {
    let start = time::Instant::now();
    marketplace::register_merchant("2vxsx-fae".into());
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
}
