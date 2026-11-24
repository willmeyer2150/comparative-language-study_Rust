# Write First Program in Rust

Experiment ID: E02
Date added: 2026-11-23
Status: Complete

**Question:** What are you trying to find out?

What is the general workflow to start a program, write it, compile it and run it for output?

**Prediction:** What do you expect, and why?

I expect "Hello World" to be a standard program that is easy to write in seconds. Additionally, given what I have heard about Rust, it is very fast and efficient, meaning I expect build, compile and execution to be a seamless step that just works as expected.

**Setup:** Boot up vsCode, create a project directory, write the code, build and run it.

Rust uses cargo run as its compile/build manager.

**Procedure:** Numbered steps someone else could repeat.

There are multiple ways to start a project. An easy method:

1. From the terminal, go to the directory where you will house your project.
2. Type: `cargo new my_project` to begin a new Cargo package with a starter "Hello, world!" binary.
3. Move into the new directory: `cd my_project`
4. Open it in VS Code: `code .`
5. Write your code in a `src/main.rs`. 
6. Cargo is the package manager and build tool. 
7. `cargo run` builds the package if needed, then runs it.
8. Check for common issues while working:

```
cargo check - checks whether the project compileswithout producing the final executable.

cargo fmt - formats the source

cargo clippy runs additional lint checks.
```

**Evidence:** Commands, code, compiler output, runtime output, timing, or screenshots.

```
fn main() {
println!("Hello, world!");
}
```

**Observation:** What happened, stated separately from interpretation.

- `fn main()` declares the program's entry point function. It is similar to c++'s main declaration, but `fn` is Rust's function declaration keyword. main is the function name.
- `println!` is a macro as the `!` indicates. It prints formatted text followed by a newline.

Of note, other similarities to c++ include parenthesis at the end of function names, use of curly braces to indicate scope and use of `;` to mark the end of code lines.

- `()` are used for macro invocation
- `{}` are used for the function body
- `;` is not used for every expression. Rust also has expressions that return values without trailing semicolons. 

**Conclusion:** What does the evidence suggest about your question?

While there are similarities to other languages that I know, there will be a learning curve to memorizing the new syntax and its semantics.

**Next step:** What small follow-up would resolve an uncertainty or extend the result?

Hello, world! is a good place to start, but by itself, offers no depth of study. Now that we have this baseline understanding of getting up and running with Rust, it is time to move on to more involved experiments that run comparative weight exercises such as typing, ownership, memory management and error handling.
