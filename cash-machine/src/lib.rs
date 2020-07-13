mod cash_machine;

use cash_machine::CashMachine;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Index;

pub fn handle(request: String) -> String {
    let atm = CashMachine::create(vec![100, 50, 20, 10, 5, 2, 1]);
    return format!("{}", atm.get(request.parse().unwrap()).unwrap());
}

#[test]
fn handle_test() {
    assert_eq!("0", handle(String::from("23")));
}

#[test]
fn map_test() {
    let mut map = HashMap::new();
    map.insert(String::from("key"), 50);
    assert_eq!(&50, map.get("key").unwrap());
}
