fn is_div(x: uint, n :uint) -> bool {
  for i in range(1, n + 1) {
    if x % i != 0 {
      return false;
    }
  }
  return true;
}


fn main() {
  let mut i = 1;

  loop {
    if is_div(i, 20) {
      println(i.to_str());
      return;
    }
    i += 1
  }
}
