mod cash_machine;

use cash_machine::CashMachine;
use serde::Serialize;
use std::panic;

// Oriented to RFC7807
#[derive(Serialize)]
pub struct ProblemDetail {
    title: String,
    detail: String,
    status: u32,
}

impl ProblemDetail {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}

//panic::set_hook(Box::new(|_| {
//    println!("Custom panic hook");
//}));

pub fn handle(json_request: String) -> String {
    let request: serde_json::Value = serde_json::from_str(&json_request).unwrap_or_else(|error| {
        panic!(ProblemDetail {
            title: String::from("Invalid json"),
            detail: format!(
                "Parsing incoming json failed with: {}: {}",
                error, json_request
            ),
            status: 400
        }
        .to_string());
    });

    let amount = request["amount"].as_u64().unwrap_or_else(|| {
        panic!(ProblemDetail {
            title: String::from("Not a number"),
            detail: format!(
                "Converting 'amount' to unsigned integer failed: {}",
                request["amount"].to_string()
            ),
            status: 400
        }
        .to_string());
    }) as u32;

    let atm = CashMachine::create(vec![100, 50, 20, 10, 5, 2]);
    let notes = atm.get(amount).unwrap_or_else(|error| {
        panic!(ProblemDetail {
            title: String::from("Processing failed"),
            detail: format!("Could not perform requested operation: {}", error),
            status: 500
        }
        .to_string());
    });

    return serde_json::json!({ "notes": &notes }).to_string();
}

#[test]
#[should_panic(
    expected = "{\"detail\":\"Could not perform requested operation: Unable to split amount 23 to available notes [100, 50, 20, 10, 5, 2]\",\"status\":500,\"title\":\"Processing failed\"}"
)]
fn handle_invalid_split_test() {
    handle(serde_json::json!({"amount": 23}).to_string());
}

#[test]
#[should_panic(
    expected = "{\"detail\":\"Converting \'amount\' to unsigned integer failed: \\\"this is not a number\\\"\",\"status\":400,\"title\":\"Not a number\"}"
)]
fn handle_invalid_amount_test() {
    handle(serde_json::json!({"amount": "this is not a number"}).to_string());
}

#[test]
#[should_panic(
    expected = "{\"detail\":\"Parsing incoming json failed with: expected value at line 1 column 1: ah, this is not json\",\"status\":400,\"title\":\"Invalid json\"}"
)]
fn handle_invalid_json_test() {
    handle(String::from("ah, this is not json"));
}

#[test]
fn handle_valid_test() {
    let request = serde_json::json!({"amount": 22 });
    let result: serde_json::Value = serde_json::from_str(&handle(request.to_string())).unwrap();
    let notes = &result["notes"];
    assert!(notes["100"].is_null());
    assert!(notes["50"].is_null());
    assert_eq!(1, notes["20"]);
    assert!(notes["10"].is_null());
    assert!(notes["5"].is_null());
    assert_eq!(1, notes["2"]);
    assert!(notes[""].is_null());
}
