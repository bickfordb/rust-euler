use std::num;

fn is_prime(check: uint) -> bool {
  if check == 2 {
    return true
  }
  if check % 2 == 0 {
    return false;
  }

  for i in range(3, num::sqrt(check as f64) as uint + 1) {
    if check % i == 0 {
      return false;
    }
  }
  return true;
}

fn main() {
  let mut check : uint = 1;
  let mut i : uint = 0;
  let nth : uint = 10001;
  while i < nth {
    check += 1;
    if (is_prime(check)) {
      i += 1;
    }
  }
  println(check.to_str())
}
