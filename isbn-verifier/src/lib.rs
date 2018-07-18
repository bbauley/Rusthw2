/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
  let v: Vec<&str> = isbn.split('-').collect();   //Splits the str into chunks seperated by '-'
  let length = v.len();
  let mut is_valid =      
    match length {      //Checks the length of the vector and each string in each element
      1 => true,   
      4 => {
        let mut valid = true;
        if v[0].len() != 1 || v[1].len() != 3
          || v[2].len() != 5 || v[3].len() != 1 {
            valid = false;
        }
        valid
      },
      _ => false
    };
  if is_valid {
    let mut string = "".to_string();
    for element in v.iter() {
      string.push_str(element); //Builds the string back from the vector
    }
    is_valid = equation(&string);
  }
  is_valid
}

/// Traverses through the string checking valid characters 
/// as well as checking to see if the isbn returns 0
fn equation(isbn: &str) -> bool {
  let mut valid = false;  
  let mut result = 0;
  let mut invalid_char = false;
  let length = isbn.len();

  if length == 10 {
    let mut multiplier = 10;
    for (i, letter) in isbn.chars().enumerate() {    //Loop through each character and add value to result
      if !letter.is_digit(10) && letter != 'X' {
        invalid_char = true;    //Invalid character stop traversing
        break;
      }
      else if letter == 'X' && i != length - 1 {
        invalid_char = true;    //If X is not at the end stop traversing
        break;
      }
      else {
        result = 
          match letter {
            'X' => result + (10 * multiplier),
             _  => result + (letter.to_digit(10).unwrap_or(0) * multiplier)
          };
        multiplier = multiplier - 1;
      }
    }
    result = result % 11;
    if !invalid_char && result == 0 {
        valid = true;
    }
  }
  valid
}
