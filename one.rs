
fn main() {
  println("one!");
  let mut s : int = 0;
  let mut i : int = 1;
  loop {
    if i >= 1000 {
      break
    }
    if ((i % 3) == 0)  {
      s += i;
    } else if ((i % 5) == 0) {
      s += i;
    } else {
    }
    i += 1;
    println("s:" +  s.to_str());
  }
  println("result: " +  s.to_str());
}

