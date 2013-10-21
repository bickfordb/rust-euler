
use std::hashmap::HashSet;


fn main() {
  let n : u64 = 600851475143;
  let mut p : u64 = 2;
  let mut i : u64;
  let mut h = HashSet::new::<u64>();
  h.insert(1);
  h.insert(2);
  while (p * p) < n {
    i = p + p;
    while (i * i) < n {
      h.insert(i);
      i += p;
    }
    p += 1;
    while h.contains(&p) {
      p += 1;
    }
    println("p" + p.to_str());
  }
  i = p - 1;
  println("Second loop");
  while i > 0 {
    if !h.contains(&i) && ((n % i) == 0u64) {
      println("largest: " + i.to_str());
      break;
    }
    i -= 1;
  }
}
