
fn is_palindrome(n : uint) -> bool {
  let s = n.to_str();
  let mut result = true;
  let k = s.len();
  for i in range(0, (k / 2)) {
    if s[i] != s[(k - 1) - i] {
      result = false;
      break;
    }
  }
  return result;
}

fn main() {
  let mut i = 999;
  let mut max : uint = 0;
  while i >= 100 {
    let mut j = 999;
    while j >= 100 {
      let k = i * j;
      if is_palindrome(k) && k > max {
        max = k;
      }
      j -= 1;
    }
    i -= 1;
  }
  println(max.to_str())
}
