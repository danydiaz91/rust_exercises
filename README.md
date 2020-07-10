# Rust Exercises

## Exercise 1: 
[Move Semantics](https://github.com/rust-lang/rustlings/tree/rustlings-1#move-semantics)
## Exercise 2: 
### Part 1: Ignore error handling here
1. Create a method that reads a file that contains words separated by spaces.
2. Create a 2nd method that takes the contents of the file as input and creates a hash set with the words in the file.
3. Create a 3rd method that receives 2 parameters, the hashset from method 2, and a vector of strings. For each string in the vector, print "Yes" if it's present in the hashset, otherwise, print "No".
4. From the main method, call methods 1 and 2 once, and call method #3 several times with the same hash set and with different input vectors.
### Part 2: Don't panic!
1. Add error handling to the code.