pub fn find() -> Option<u32> {
  let mut result = 0u32;
  for n in 1u32..1001 { //m and n need to be > 0
    for m in n..1001 {  
      let a = m.pow(2) - n.pow(2);  
      let b = 2 * m * n;          //Set a, b, and c to their values
      let c = m.pow(2) + n.pow(2);
      if a + b + c == 1000 {    //Store product if their sum is equal to 1000
        result = a * b * c;
      }
    }
  }
  if result == 0 {
    None
  }
  else {
    Some(result)
  }
}