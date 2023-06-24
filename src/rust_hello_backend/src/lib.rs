mod merchant;
mod cart;
mod product;

/*
todo - Add tests
 */

#[test]
fn check_candid_interface() {
    use candid::utils::{service_compatible, CandidSource };
    use std::path::Path;
    use crate::merchant::Merchant;

    // Generate the service definition (candid format)
    candid::export_service!();
    let new_interface = __export_service();

    // Write the generated service definition to a file for dfx deploy
    let mut buffer = new_interface.as_bytes();
    std::fs::write("backend.did", buffer)
        .expect("Issue generating backend.did file");

    service_compatible(
        CandidSource::Text(&new_interface),
        CandidSource::File(Path::new("backend.did")),
    ).unwrap();
}