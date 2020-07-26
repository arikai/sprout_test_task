const M: i64 = 42;
const P: i64 = 128;
const T: i64 = 1024;

fn business_logic(a: bool, b: bool, c: bool,
                  d: f64, e: i64, f: i64) -> (i64, f64) {
    let h: i64;
    let k: f64;

    h =
        match (a, b, c) {
            (true, true, false) => M,
            (true, true, true) => P,
            (false, true, true) => T,
            _ => panic!("Bad inputs")                   // TODO: return error
        };

    k =
        match h {
            M => d + (d * (e as f64) / 10.),
            P => d + (d * (e - f) as f64 / 25.5),
            T => d - (d * f as f64 / 30.),
            _ => panic!("Bad inputs")
        };

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
