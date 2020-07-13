use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct CashMachineError {
  pub message: String,
}

impl fmt::Display for CashMachineError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    return write!(f, "CashMachineError({})", self.message);
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
        return value;
      }
      let count = std::cmp::min(value / note, std::u32::MAX);
      if count == 0 {
        // This note type is not part of the result set
        return value;
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
        "Unable to split amount {} to available notes {:?}",
        amount, self.notes
      ),
    });
  }
}

#[test]
fn get_valid_test() {
  let notes = CashMachine::create(vec![100, 50, 20, 10, 5])
    .get(175)
    .unwrap();
  assert_eq!(4, notes.len());
  assert_eq!(&1, notes.get(&100).unwrap());
  assert_eq!(&1, notes.get(&50).unwrap());
  assert_eq!(&1, notes.get(&20).unwrap());
  assert_eq!(&1, notes.get(&5).unwrap());
}

#[test]
fn get_zero_amount_test() {
  let notes = CashMachine::create(vec![10, 5]).get(0).unwrap();
  assert_eq!(0, notes.len());
}

#[test]
fn unable_to_distribute_test() {
  assert_eq!(
    CashMachineError {
      message: String::from("Unable to split amount 4 to available notes [10, 5]")
    },
    CashMachine::create(vec![10, 5]).get(4).err().unwrap()
  );
}

#[test]
fn format_error_test() {
  assert_eq!(
    String::from("CashMachineError(bad)"),
    format!(
      "{}",
      CashMachineError {
        message: String::from("bad")
      }
    )
  )
}
