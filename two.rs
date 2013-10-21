
fn main() {
  let mut prev : uint = 1;
  let mut term : uint = 2;
  let mut sum : uint = 0;
  loop {
    if term % 2 == 0 {
      sum += term;
    }
    let next = term + prev;
    prev = term;
    term = next;
    if term > 4000000 {
      break;
    }
  }
  println("sum: " + sum.to_str());
}
