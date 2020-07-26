const M: i64 = 42;
const P: i64 = 128;
const T: i64 = 1024;

use counted_array::*;

type BoolFun = fn(bool, bool, bool) -> Option<i64>;
type CalcFun = fn(i64, f64, i64, i64) -> Option<f64>;

counted_array!(
  const CUSTOM_RULES: [(Option<BoolFun>, Option<CalcFun>); _] = [
    (
      None,
      Some(|h, d, e, _f| {
        match h {
          P => Some(2. * d + (d * e as f64 / 100.)),
          _ => None
        }
      })),
    (
      Some(|a, b, c| {
        match (a, b, c) {
          (true, true, false) => Some(T),
          (true, false, true) => Some(M),
          _ => None
        }
      }),
      Some(|h, d, e, f| {
        match h {
          M => Some(f as f64 + d + (d * e as f64 / 100.)),
          _ => None
        }
      })
    )
  ]
);

fn try_custom_bool_rules(a: bool, b: bool, c: bool) -> Option<i64> {
  for (bool_rule, _crule) in CUSTOM_RULES.iter() {
    match bool_rule.and_then(|fun| { fun(a, b, c) }) {
      None => continue,
      Some(h) => return Some(h)
    }
  }
  None
}

fn try_custom_calc_rules(h: i64, d: f64, e: i64, f: i64) -> Option<f64> {
  for (_brule, calc_rule) in CUSTOM_RULES.iter() {
    match calc_rule.and_then(|fun| { fun(h, d, e, f) }) {
      None => continue,
      Some(k) => return Some(k)
    }
  }
  None
}

fn business_logic(a: bool, b: bool, c: bool,
                  d: f64, e: i64, f: i64) -> (i64, f64) {
  let h: i64;
  let k: f64;

  h =
    try_custom_bool_rules(a, b, c)
    .unwrap_or_else(|| {
      match (a, b, c) {
        (true, true, false) => M,
        (true, true, true) => P,
        (false, true, true) => T,
        _ => panic!("Bad inputs")                   // TODO: return error
      }
    });

  k =
    try_custom_calc_rules(h, d, e, f)
    .unwrap_or_else(|| {
      match h {
        M => d + (d * (e as f64) / 10.),
        P => d + (d * (e - f) as f64 / 25.5),
        T => d - (d * f as f64 / 30.),
        _ => panic!("Bad inputs")
      }
    });

  (h, k)
}

fn main() {
  println!("{:?}, {:?}, {:?}",
           business_logic(true, true, false, 1.0, 2, 3),
           business_logic(true, true, true, 1.0, 2, 3),
           business_logic(false, true, true, 1.0, 2, 3)
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_rule_1() {
    assert_eq!((42, 1.2),
               business_logic(true, true, false, 1.0, 2, 3))
  }

  #[test]
  fn default_rule_2() {
    assert_eq!((128, 0.9607843137254902),
               business_logic(true, true, true, 1.0, 2, 3))
  }

  #[test]
  fn default_rule_3() {
    assert_eq!((1024, 0.9),
               business_logic(false, true, true, 1.0, 2, 3))
  }
}
