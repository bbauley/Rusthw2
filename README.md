# Rust hw2

### Lessons Learned

- You need to call an iterator on a str 
- Chaining an iterator with an enumerate() function gives back each element along with its "index"
- If statements need to return a () when above other code
- Needed to change design to update variables in if statements instead of returning because of bullet 3
- Rust needs to know the *specific type* of integer(u32) when using the pow function
- Syntax for working with enums and structs
- Modifying code structure to work with parameters given
  - returning Self
  - not being able to modify parameter

### Things that didn't go well

The functions that needed to be written and tested went well for the most part.....*except*
- Design
  - If there are if statements above code, Rust is expecting us to return a (). My habit has been to have if conditions at the beginning of each function that return if something is incorrect(base cases). Since I couldn't do that, I spent a lot of my time trying to figure out what the cleanest alternative would be.
  - I needed to traverse through a string and *exit* the loop as soon as I found a letter that was not valid. I couldn't figure out how to iterate through a str with a while loop so I used a for loop and used break if a condition held true.
  - If I had more time I would go over each function and spend some time rewritting the code for readability and efficiency.
