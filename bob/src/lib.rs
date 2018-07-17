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
    let remove_whitespace = message.split_whitespace(); //Remove any whitespaces
    let clone = remove_whitespace.clone();
    let num_of_words = remove_whitespace.count();   //TODO NEED TO CLEAN UP CODE
    for (i, word) in clone.enumerate() {  //Loop through each word
        let num_of_letters = word.len();
        for (j, letter) in word.chars().enumerate() {   //Loop through each character in each word 

            /* Checks for the yelling condition */
            if yelling && letter.is_alphanumeric() {
                if letter.is_lowercase() {
                    println!("SET YELLING TO FALSE");
                    yelling = false;
                }
            }

            /*Check for question condition */
            if question && j == num_of_words - 1 {  
                if i == num_of_letters - 1 {        //Check the last word and letter for ?
                    if letter != '?' {
                        println!("SET QUESTION TO FALSE");
                        question = false;
                    }
                }
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
