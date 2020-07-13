mod cash_machine;

use cash_machine::CashMachine;

pub fn handle(json_request: String) -> String {
    let request: serde_json::Value = serde_json::from_str(&json_request).unwrap();
    let amount = request["amount"].as_u64().unwrap() as u32;
    let atm = CashMachine::create(vec![100, 50, 20, 10, 5, 2, 1]);
    let notes = atm.get(amount).unwrap();
    let result = serde_json::json!({ "notes": &notes }).to_string();
    return result;
}

#[test]
fn handle_test() {
    let request = serde_json::json!({"amount": 23 });
    let result: serde_json::Value = serde_json::from_str(&handle(request.to_string())).unwrap();
    let notes = &result["notes"];
    assert!(notes["100"].is_null());
    assert!(notes["50"].is_null());
    assert_eq!(1, notes["20"]);
    assert!(notes["10"].is_null());
    assert!(notes["5"].is_null());
    assert_eq!(1, notes["2"]);
    assert_eq!(1, notes["1"]);
    assert!(notes[""].is_null());
}
