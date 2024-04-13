/*
URLs:
  Frontend canister via browser
    ICP_RUST_workshop_frontend:
      - http://127.0.0.1:4943/?canisterId=br5f7-7uaaa-aaaaa-qaaca-cai
      - http://br5f7-7uaaa-aaaaa-qaaca-cai.localhost:4943/
  Backend canister via Candid interface:
    ICP_RUST_workshop_backend: http://127.0.0.1:4943/?canisterId=bw4dl-smaaa-aaaaa-qaacq-cai&id=be2us-64aaa-aaaaa-qaabq-cai
*/

use std::cell::RefCell;

thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn dodaj_wpis(wpis: String) {
    WPISY.with(|wpisy| {
        let mut mutable_wpisy = wpisy.borrow_mut();
        mutable_wpisy.push(wpis);
    });
}

ic_cdk::export_candid!();
