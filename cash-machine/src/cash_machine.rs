use std::fmt;

#[derive(Debug)]
pub struct CashMachineError {
  pub message: String,
}

impl fmt::Display for CashMachineError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "CashMachineError")
  }
}

pub struct CashMachine {
  pub notes: Vec<u32>,
}

impl CashMachine {
  pub fn create(notes: Vec<u32>) -> CashMachine {
    return CashMachine { notes: notes };
  }

  pub fn get(&self, amount: u32) -> Result<u32, CashMachineError> {
    return Ok(self.notes.iter().fold(amount, |value, note| {
      let count = std::cmp::min(value / note, u32::MAX);
      let residual = value - note * count;
      println!("Print {} notes of amount {}", count, note);
      return residual;
    }));
  }
}

#[test]
fn error_test() {
  assert_eq!(
    String::from("CashMachineError"),
    format!(
      "{}",
      CashMachineError {
        message: String::from("bad")
      }
    )
  )
}
