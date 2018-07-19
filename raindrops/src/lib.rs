pub fn raindrops(n: usize) -> String {

  let mut result = "".to_string();
  let mut factor = false;
  if n % 3 == 0 {
    factor = true;
    result.push_str("Pling");
  }
  if n % 5 == 0 {
    factor = true;
    result.push_str("Plang");
  }
  if n % 7 == 0 {
    factor = true;
    result.push_str("Plong");
  }

  if factor {
    result
  }
  else {
    n.to_string()
  }
}