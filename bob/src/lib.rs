
pub fn reply(message: &str) -> &str {

    let mut yelling = true;
    let mut no_letters = true;
    let mut question = true;
    let mut nothing = true;
    let remove_whitespace = message.trim(); //Remove any whitespaces
    let num_of_letters = remove_whitespace.len();   
    for (i, letter) in remove_whitespace.chars().enumerate() {   //Loop through each letter
        nothing = false;
        /* Checks for the yelling condition */
        if yelling && letter.is_alphabetic() {
            no_letters = false;
            if letter.is_lowercase() {
                yelling = false;
            }
        }
        /*Check for question condition */
        if question && i == num_of_letters - 1 {  
            if letter != '?' {
                question = false;
            }
        }
    } 
    if no_letters {     //Need to set yelling to false if no letters in string
        yelling = false;
    }

    if nothing {
        "Fine. Be that way!"
    }
    else if yelling && question {
        "Calm down, I know what I'm doing!"
    }
    else if yelling {
        "Whoa, chill out!"
    }
    else if question { 
        "Sure."
    }
    else {
        "Whatever."
    }
}
