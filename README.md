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

## Exercise 3:
### Part 1:
Create a program to manage some classes for colleges and high schools:

1. The high school student is represented by a name, a grade and the high school where he is enrolled
2. The college student is represented by a name a grade and the program he is cursing
3. A class is represented by a name, the professor name and a list of students, either high school or college (a class can only have either high school or college students at the same time)
4. The class must provide a function to enroll students
You can structure this as you wish (everything in main, using mods, etc...)
Then on the main function create 3 HS students and 3 college students enroll them all in two classes.

### Part 2
1. The class must provide a function that validates if all the students are enrolled on either the same program or high school 
2. The class must provide a function that ranks the students by grade (higher to lower) and returns an iterable of the students (The class should still contains the original order of the students as they enrolled)

Then on the main function, print the result of the validation and call the ranking function, printing the student names ranked for both classes
after that, you must be able to validate on the main function if two college students are on the same program using the == operator and print this result (without using the class)