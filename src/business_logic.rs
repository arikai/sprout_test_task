use serde_derive::Serialize;
use std::fmt;

const M: i64 = 42;
const P: i64 = 128;
const T: i64 = 1024;

use counted_array::*;

type BoolFun = fn(bool, bool, bool) -> Option<i64>;
type CalcFun = fn(i64, f64, i64, i64) -> Option<f64>;

pub struct CustomRule {
  bool_fun: Option<BoolFun>,
  calc_fun: Option<CalcFun>
}

#[derive(Debug, PartialEq, Serialize)]
pub struct LogicResult {
  h: i64,
  k: f64
}

#[derive(Debug, PartialEq)]
pub enum ExecutionError {
  NoRuleError
}

impl std::error::Error for ExecutionError {}

impl fmt::Display for ExecutionError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let printable = match *self {
      ExecutionError::NoRuleError => "No rule found"
    };
    write!(f, "{}", printable)
  }
}

const CUSTOM_RULE_1: CustomRule =
  CustomRule{
    bool_fun: None,
    calc_fun: Some(|h, d, e, _f| {
      match h {
        P => Some(2. * d + (d * e as f64 / 100.)),
        _ => None
      }
    })};

const CUSTOM_RULE_2: CustomRule =
  CustomRule{
    bool_fun: Some(|a, b, c| {
      match (a, b, c) {
        (true, true, false) => Some(T),
        (true, false, true) => Some(M),
        _ => None
      }
    }),
    calc_fun: Some(|h, d, e, f| {
      match h {
        M => Some(f as f64 + d + (d * e as f64 / 100.)),
        _ => None
      }
    })
  };

counted_array!(
  const CUSTOM_RULES: [CustomRule; _] = [
    CUSTOM_RULE_1,
    CUSTOM_RULE_2
  ]
);

const DEFAULT_RULE: CustomRule =
  CustomRule{
    bool_fun: Some(|a, b, c| {
      match (a, b, c) {
        (true, true, false) => Some(M),
        (true, true, true) => Some(P),
        (false, true, true) => Some(T),
        _ => None
      }
    }),
    calc_fun: Some(|h, d, e, f| {
      match h {
        M => Some(d + (d * (e as f64) / 10.)),
        P => Some(d + (d * (e - f) as f64 / 25.5)),
        T => Some(d - (d * f as f64 / 30.)),
        _ => None
      }
    })
  };

fn apply_bool_rules(
  a: bool, b: bool, c: bool,
  rules: &[CustomRule]) -> Result<i64, ExecutionError>
{
  for rule in rules.iter().chain([DEFAULT_RULE].iter()) {
    match rule.bool_fun.and_then(|fun| { fun(a, b, c) }) {
      None => continue,
      Some(h) => return Ok(h)
    }
  }
  Err(ExecutionError::NoRuleError)
}

fn apply_calc_rules(
  h: i64, d: f64, e: i64, f: i64,
  rules: &[CustomRule]) -> Result<f64, ExecutionError>
{
  for rule in rules.iter().chain([DEFAULT_RULE].iter()) {
    match rule.calc_fun.and_then(|fun| { fun(h, d, e, f) }) {
      None => continue,
      Some(k) => return Ok(k)
    }
  }
  Err(ExecutionError::NoRuleError)
}

pub fn execute(
  a: bool, b: bool, c: bool,
  d: f64, e: i64, f: i64
) -> Result<LogicResult, ExecutionError> {
  execute_with_rules(a, b, c, d, e, f, &CUSTOM_RULES)
}

fn execute_with_rules(
  a: bool, b: bool, c: bool,
  d: f64, e: i64, f: i64,
  rules: &[CustomRule]
) -> Result<LogicResult, ExecutionError> {
  let h = apply_bool_rules(a, b, c, rules)?;
  let k = apply_calc_rules(h, d, e, f, rules)?;
  Ok(LogicResult{ h, k })
}

#[cfg(test)]
mod tests {
  use super::*;

  const D: f64 = 1.0;
  const E: i64 = 2;
  const F: i64 = 3;

  #[test]
  fn default_rules() {
    assert_eq!(Ok(LogicResult{ h: 42, k: 1.2}),
               execute_with_rules(true, true, false, D, E, F, &[]));
    assert_eq!(Ok(LogicResult{ h: 128, k: 0.9607843137254902 }),
               execute_with_rules(true, true, true, D, E, F, &[]));
    assert_eq!(Ok(LogicResult{ h: 1024, k: 0.9 }),
               execute_with_rules(false, true, true, D, E, F, &[]));
    assert_eq!(Err(ExecutionError::NoRuleError),
               execute_with_rules(false, false, false, D, E, F, &[]));
  }

  #[test]
  fn custom_rule_1() {
    assert_eq!(Ok(LogicResult{ h: 128, k: 2.02 }),
               execute_with_rules(true, true, true, D, E, F, &[CUSTOM_RULE_1]));
  }

  #[test]
  fn custom_rule_2() {
    assert_eq!(Ok(LogicResult{ h: 1024, k: 0.9 }),
               execute_with_rules(true, true, false, D, E, F, &[CUSTOM_RULE_2]));
    assert_eq!(Ok(LogicResult{ h: 42, k: 4.02 }),
               execute_with_rules(true, false, true, D, E, F, &[CUSTOM_RULE_2]));
  }

  #[test]
  fn with_all_custom_rules() {
    assert_eq!(Ok(LogicResult{ h: 128, k: 2.02 }),
               execute(true, true, true, D, E, F));
    assert_eq!(Ok(LogicResult{ h: 1024, k: 0.9 }),
               execute(true, true, false, D, E, F));
    assert_eq!(Ok(LogicResult{ h: 42, k: 4.02 }),
               execute(true, false, true, D, E, F));
  }
}
