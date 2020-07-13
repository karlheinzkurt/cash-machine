use std::collections::HashMap;
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

  pub fn get(&self, amount: u32) -> Result<HashMap<u32, u32>, CashMachineError> {
    let mut result = HashMap::new();
    let residual = self.notes.iter().fold(amount, |value, note| {
      if amount == 0 {
        // Nothing to do here since the amount of requested money is 0
        return amount;
      }
      let count = std::cmp::min(value / note, std::u32::MAX);
      if count == 0 {
        // This note type is not part of the result set
        return amount;
      }
      result.insert(*note, count);
      return value - note * count;
    });
    if residual == 0 {
      // We were able to to split the requested amount of money to available notes
      return Ok(result);
    }
    return Err(CashMachineError {
      message: format!(
        "Unable to distribute requested amount {} to available notes (todo: print available notes)",
        amount
      ),
    });
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
