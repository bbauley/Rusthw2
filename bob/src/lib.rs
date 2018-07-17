/*
Bob is a lackadaisical teenager. In conversation, his responses are very limited.

Bob answers 'Sure.' if you ask him a question.

He answers 'Whoa, chill out!' if you yell at him.

He answers 'Calm down, I know what I'm doing!' if you yell a question at him.

He says 'Fine. Be that way!' if you address him without actually saying
anything.

He answers 'Whatever.' to anything else.
*/

pub fn reply(message: &str) -> &str {

    let mut yelling = true;
    let mut question = true;
    let remove_whitespace = message.trim(); //Remove any whitespaces
    println!("remove_whitespace: {}", remove_whitespace);
    let num_of_letters = remove_whitespace.len();   
    for (i, letter) in remove_whitespace.chars().enumerate() {   //Loop through each letter

        /* Checks for the yelling condition */
        if yelling && letter.is_alphanumeric() {
            if letter.is_lowercase() {
                println!("SET YELLING TO FALSE");
                yelling = false;
            }
        }

        /*Check for question condition */
        if question && i == num_of_letters - 1 {  
            if letter != '?' {
                println!("SET QUESTION TO FALSE");
                question = false;
            }
        }
    } 

    if yelling && question {
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
