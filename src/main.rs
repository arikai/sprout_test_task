mod business_logic;

fn main() {
  println!("{:?}, {:?}, {:?}",
           business_logic::execute(true, true, false, 1.0, 2, 3),
           business_logic::execute(true, true, true, 1.0, 2, 3),
           business_logic::execute(false, true, true, 1.0, 2, 3)
  );
}
