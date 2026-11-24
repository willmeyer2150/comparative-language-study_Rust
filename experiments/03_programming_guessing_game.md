# Title

Experiment ID: E03
Date added: 2026-11-23
Status: In progress

**Question:** What are you trying to find out?

What fundamentals can be learned by programming a classic example: guess_game

**Prediction:** What do you expect, and why?

I expect to be introduced to variables, loops, assignment and basic logic implementation using conditional statements.

**Setup:** Create a new project in the common-comparison folder.

**Procedure:** Numbered steps someone else could repeat.

1. Use command: cargo new --vcs none guessing_game
- --vcs none eliminates initializing a git repo

**Evidence:** Commands, code, compiler output, runtime output, timing, or screenshots.

The command, `cargo new` generates a "Hello, world!" program. 

`cargo run` compiles and runs the project. This comes in handy when you need ot rapidly iterate on a project. It makes it so you can test each iteration before moving on to the next one.

**Observation:** What happened, stated separately from interpretation.

Building this classic example in rust, I couldn't help but compare the process and execution to building the same program in C++. In fact, I think I built this program in my first coding class at the begining of the CS major. 

Like C++, the beginning of the file has a way to declare a collection of names that you can automatically bring into scope. While C++ uses #includes, Rust uses `use`. There are several different preludes that bring names into scope of every module in a crate. An example used in this program is std::io which provides a number of useful features including the ability to accept user input.

Storing Values with Variables:
In Rust, variables are immutable by default. Once we give a variable a value, that value won't change. A variable is declared with `let`. Adding `mut` before a variable name makes it mutable.

Associated functions:
You can build a function that is implemented on a type. In this program's case, we can declare it as type string using: `String::new()`. 

Stdin from the io module:
`std::io` provides input/output functionality. Like in C++, if we didn't use std::io as a prelude, we could still use the function call as `std::io::stdin`. `stdin` returns an instance of `std::io:;stdin which is a type that represents a handle to the standard input of my terminal.

`read_line()` has the job to take whatever the user types into stdin and append that into a string (w/o overwriting its contents). The string argument needs to be mutable so that the method can change the string's content.

We use `&mut guess` to indicate that this argument is a reference which allows a way to let multiple parts of program access one piece of data w/o needing to copy that data into memory multiple times. References are immutable by default.

Result
An instance of Result has an expect method that you can call. If instance of Result is Err value, .expect will cause program to crash, and display the message that you passed as an argument to expect. If .read_line method returns Err, it is likely a result of the error coming form the OS. If this instance of Result is an ok value, expect will take on the return value that ok is holding and return it; in this instance, it is the number of bytes in the user's input.

Shadowing a variable.
Rust allows you to shadow a variable which means you shadow the previous value of the variable with a new one. Shadowing lets you reuse the variable name rather than forcing you to create a new one. 


**Conclusion:** What does the evidence suggest about your question?

Like C++, there are elements to Rust, and every programming language, that are required to make a program operate properly and efficiently. In the case of the guessing game, we have the opportunity to bring names into scope, declare variables, learn about mutable vs immutable varialbes, how to read user input. Printing output. Changing variable type with shadowing, Create conditional logic with match and ordering, and breaking loops.

**Next step:** What small follow-up would resolve an uncertainty or extend the result?

A redo of the programming without the tutorial would be a useful next step to solidify the new syntax and understand the control flow of the program. Then refining my experiment capture and do a true comparison to my other learned languages. Additionally, filling in the gaps on the report.md about the topics covered so far. 

I also could refine the program with more error handling for that experience. 
