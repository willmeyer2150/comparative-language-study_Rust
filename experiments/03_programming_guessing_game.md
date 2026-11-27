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

1. Create a Cargo project
2. Add `rand` dependency
3. Write the program per the tutorial
4. Run `cargo check` or `cargo build` to incrementally ensure the program compiles
5. Run `cargo run`
6. Enter several kinds of guesses to test funcitonaliy

**Evidence:** Commands, code, compiler output, runtime output, timing, or screenshots.

The command, `cargo new` generates a "Hello, world!" program. 

Successful build of the project:
```
willm@Wills-MacBook-Pro src % cargo build
   Compiling guessing_game v0.1.0 (/Users/willm/school_projects/cs460_programming_languages/projects/comparative-language-study/programs/common-comparison/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
```

Successful run of the project with winning state plus too low or too high guess.
```
willm@Wills-MacBook-Pro src % cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `/Users/willm/school_projects/cs460_programming_languages/projects/comparative-language-study/programs/common-comparison/guessing_game/target/debug/guessing_game`
Guess the number!
Please input your guess.
50
You guessed: 50
Too big!
Please input your guess.
40
You guessed: 40
Too small!
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
48
You guessed: 48
Too small!
Please input your guess.
49
You guessed: 49
You win!
```

Demonstration of non-numeric input:
```
willm@Wills-MacBook-Pro src % cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `/Users/willm/school_projects/cs460_programming_languages/projects/comparative-language-study/programs/common-comparison/guessing_game/target/debug/guessing_game`
Guess the number!
Please input your guess.
a
Please input your guess.
k
Please input your guess.
f
Please input your guess.
f
Please input your guess.
```

Compiler Errors:

Mainly due to improper entering of syntax such as a missing semi-colon and missing brace to close a code block. Given I was following a tutorial, this was just an oversite. When I code myself without a model to follow, I will capture more useful compiler errors that reflect my coding choices.

**Observation:** What happened, stated separately from interpretation.

Building this classic example in rust, I couldn't help but compare the process and execution to building the same program in C++. In fact, I think I built this program in my first coding class at the begining of the CS major. 

Like C++, the beginning of the file has a way to declare a collection of names that you can automatically bring into scope. While C++ uses #includes, Rust uses `use` to bring specific names into scope. In this case, `use std::io` allows for input/output functionality.

Storing Values with Variables:
In Rust, variables are immutable by default. Once we give a variable a value, that value won't change. A variable is declared with `let`. Adding `mut` before a variable name makes it mutable.

Associated functions:
You can build a function that is implemented on a type. In this program, `String::new()` calls an associated function. 

Stdin from the io module:
`std::io` is a module being imported and provides input/output functionality. Like in C++, if we didn't use std::io, we could still use the function call as `std::io::Stdin`. `stdin` returns an instance of `std::io:;stdin which is a type that represents a handle to the standard input of my terminal.

`read_line()` has the job to take whatever the user types into stdin and append that into a string (w/o overwriting its contents). The string argument needs to be mutable so that the method can change the string's content. It returns a `Result<usize>`.

`&mut guess` is a mutable reference. Shared references are immutable by default, but mutable references can modify the referenced value. This allows a way to let multiple parts of program access one piece of data w/o needing to copy that data into memory multiple times. In this case, `&mut guess` gives `read_line()` permission to modify the existing `Striing`.

Result
An instance of Result has an expect method that you can call. If instance of Result is Err value, .expect will `panic` and crash the program. In this case, it will display the message that you passed as an argument to expect. If .read_line method returns Err, it is likely a result of the error coming form the OS. If this instance of Result is an ok value, expect will return the value that ok is holding; in this instance, it is the number of bytes in the user's input.

`parse()` also returns a `Result`, which is why the second `match` works.

Shadowing a variable.
Rust allows you to shadow a variable which means you shadow the previous value of the variable with a new one. Shadowing lets you reuse the variable name rather than forcing you to create a new one. 


**Conclusion:** What does the evidence suggest about your question?

The guessing game program introduced many of Rust's base line components. It included:

- Variables
- Mutability
- Shadowing
- Loops
- Patten Matching
- References
- Error handling using Result

What sets Rust apart from C++ was that input conversion and possilbe failer are handled explicitly throuh `Result`.

**Next step:** What small follow-up would resolve an uncertainty or extend the result?

A redo of the programming without the tutorial would be a useful next step to solidify the new syntax and understand the control flow of the program. Then refining my experiment capture and do a true comparison to my other learned languages. Additionally, filling in the gaps on the report.md about the topics covered so far. 

I also could refine the program with more error handling for that experience. 
