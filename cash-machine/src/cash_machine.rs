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

pub struct CashMachine {
  pub stages: Vec<Box<dyn Fn(u32) -> u32>>,
}

impl CashMachine {
  pub fn create(notes: Vec<u32>) -> CashMachine {
    let mut cash_machine = CashMachine { stages: Vec::new() };
    for n in notes {
      cash_machine.stages.push(Box::new(move |value| {
        let count = std::cmp::min(value / n, u32::MAX);
        let residual = value - n * count;
        println!("Print {} notes of amount {}", count, n);

        return residual;
      }));
    }
    return cash_machine;
  }

  pub fn get(&self, amount: u32) -> Result<u32, CashMachineError> {
    let mut result: u32 = amount;
    for s in &self.stages {
      result = s(result);
    }
    return Ok(result);
  }
}
