
fn main() {
  let mut ssq : u64 = 0u64;
  let mut sqs : u64 = 0u64;
  
  for i in range(1, 101) {
    let iu : u64 = i as u64;
    ssq += iu * iu;
    sqs += iu;
  }
  sqs = sqs * sqs;
  println((sqs - ssq).to_str());
}
