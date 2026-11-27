# Comparative Language Study: Rust

## Abstract

TBD

---

## Introduction to Rust

**working note** 
Rust is an _ahead-of-time compiled_ language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

**Research Questions**
How does Rust's ownership, borrowing, and type system impact program safety, design difficulty, and expressiveness compared with C++?

Programming languages have a particularly complex job. Computer programs are rarely built as a simple set of instructions following logical step by step sequential pathways. A programming language must be equiped to meet a vast array of unique requirements. Consider programs that exhibit deeply concurrent behaviors or contain extensive event driven decisions to execute. Building a language to accomodate these requirements needs deeply thought out syntax and symantics. To raise the bar even further, languages must balance their functionality with being reliable, readable, and function efficiently in numerous environments. In this regard, Rust is intergesting and presents an opportunity to investigate the claims that it can provide system's level performance using compile time ownership, borrowing and type checks to prevent many memory safety errors while also providing a high level of memory safety in its framework.

No one would argue that Rust is an easy language to learn. While it provides strong tools in abstraction, performance, resource use, it is widely known to be a language of high effort to learn. To that end, why would you bother to learn Rust when C++ is already a highly capabable lower level language that might be considered good enough when it comes to a balanced approach to performance and readability? That question is one I will try to answer in this investigation. I will look at how Rust balances flexibility and control with its promise of better safety and reliability. It's ownership and borrowing model promises a great usefulness in demanding architectures, so we will run experiements and checks to see if Rust's notariety holds up. 

When learning a new language, the first thing you should do is get the computer to do something. Rust's documentation recommends the ubiquitous "Hello, world!" program as useful example to quickly write some code and output to the screen.

`println!("Hello, world!");`

All programs have a standard method of writing to the screen. In C++, `std::cout` essentially does the same task using the stream operator, `<<`, meaning, `insert a stream of characters`. Rust distinguishes itself in this regard by assigning the print task to a macro. The `println!` calls a Rust macro by use of the `!` symbol. It also offers a function version of print which uses the same function name, `println`, but without the `!`. Rust macros are a unique feature that allow the programmer to generate code to extend Rust syntax. 

Even in the most basic lines of code, while Rust achieves the same basic functionality as C++ in writing to standard output, the statement to execute the instruction is quite different. The syntax looks different and it carries with it a different semantical meaning. The similar functionality paired with quite different execution statements will be a consistent theme in throughout this analysis. Rust attempts to do some very unique things with its methodology that corrects some of the problems with memory leaks and security that C++ is known for. It does this by looking at what C++ already does really well, and adds its own take on how those instructions should be carried out.

---

## Methods

- Tools and versions
- experiment procedure
- evidence and comparison criteria

---

## Rust Implementation and toolchain

- compilation
- Cargo
- modules
- tests

---

## Rust's programming features

Address this section by thinking of each topic as having three layers:

1. What does Rust do?
2. Why is Rust designed this way?
3. How does this differ from C++ or Python and what consequence follows.

### Types and Values

**Concept**
Explain the Rust feature accurately.

- Rust is a statically typed: every expression and value has a type that the comiler determines or verifies before the program runs.
- Types can be made explicit or they can be inferred by the compiler.
- When many types are possible, the programmer's annotation gives `parse` its target type. Rust infers the generic type from that annotation.

**Small example**
Show the smallest code example that makes the feature visible.

```rust
// inferred as i32
let inferred = 5;

// explicitly declared as u8
let explicit: u8 = 5;

// Type annotation tells Rust that parse should produce a u32
let number: u32 = "42".parse().expect("Not a number:);
```

**Comparison**
Explain the equivalent or alternative in C++ or Python.

- C++ is also statically typed and supports inference through `auto` which is an inference that occurs at compile time.
- Python determines types dynamically at runtime.

**Consequence**
Discuss what the difference means for safety, clarity, flexibility, performance, or difficulty.

- Rust and C++ can detect many type errors before execution. Early error detection is a positive.
- Less runtime type uncertainty.
- Python allows faster experimentation, but moves more errors to runtime.
- Rust requires more annotation or conversion work that in Python.

**Evidence**
Refer to compiler behavior, program output, an experiment, or a documented limitation.

Rust Example: Successful Type Inferrence

```rust
fn main() {
    let inferred = 5;

    println!("{inferred}")
}
```

Program output:
```text
5
```

- The program compiled and ran successfully without declaring a type for `inferred`.
- Rust inferred the type of the integer literal during compilation.
- The default type for an unconstrained integer literal is `i32`.

Example: Incompatible assignment

```rust
fn main() {
    let word: &str = "hello";
    let number: i32 = word;
    println!("{}", number);
}
```

Program output:

```text
`rustc --explain E0308`.
error: could not compile `incompatible_assignment` (bin "incompatible_assignment") due to 1 previous error
```

### Primitive Types and Literals

#### Scalar Types

- A scalar type represents a single value. 
- There are four primary scalar types:

1. integers
2. floating-point numbers
3. booleans
4. characters

#### Integer Types

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| Architecture-dependent | isize | usize|

- What happens when arithmetic overflows?
- How does Rust compare with C++?
- What does Python do differently with integer size?

**Concept:** 

- Rust provides fixed-width signed and unsigned integer types.
- Signed and unsigned values are distinct.

**Small example:** 

The example tests what happens when u8 value exceeds its maximum value of 255.

```rust
fn main {
    let max_value: u8 = 255;
    let result = max_value + 1; // u8 can only hold values from 0 to 255
    println!("{max_value} + 1 = {result}");
}
```

Run in the default debug profile:

```bash
cargo run
```

Program output:

```text
attempt to add with overflow
```

- In the defaul debug profile, Rust detects the overflow and terminates the program with a panic.
- The println! statement is not reached.

Behavior when run in release mode:

```bash
cargo run --release
```

Program output: 

```text
255 + 1 = 0
```

- Cargo's default release settings disables overflow checks.
- Unsigned arithmetic wraps around.
- The u8 result becomes 0 after exceeding 255.

**Comparison:** 

- Rust provides fixed width integer types such as u8 and i32.
- C++ provides fixed width types such as std::uint8_t, when made available. Unsigned arithmetic wraps, while signed overflow has problematic or undefined behavior.
- Python integers can grow beyond ordinary machine word limits, subject to available memory, so they do not normally overflow at 255 merely because the value is an integer.

**Consequence:** 

- Rust's fixed width types make storage size and numeric range explicit, which is useful for systems programming and predictable data representation.
- Tradeoff is that programmers must choose suitable types and handle range limits.
- Rust's debug and release mode differences also mean that explicit methods such as checked_add, wrapping_add or saturating_add are preferred when overflow behavior matters.

#### Floating-Point Types

**Concept**

- Rust has two primitive types for floating-point numbers: f32 and f64.
    - f32 = 32 bits in size
    - f64 = 64 bits in size (default type). On modern CPU's, it's roughly the same speed as f32, but more precise.
- All floating-point types are signed

**Small Example**

This example demonstrates how a user can write code to mix types and specify to the compiler how to handle them.

```rust
fn main() {
    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32 (user specified)
    println!("{}", x + y as f64);
}
```

Program output:

```text
5
```

- Floating-point numbers are represented according to IEEE-754 standard.

**Comparison**



**Consequence**

#### Booleans Types

#### Characters Types

### Variables

Rust makes variables immutable by default, but sometimes a programmer may want to change that functionality for greater flexibility. Rust makes this choice to encourage the programmer to write code in a way that takes advantage of the safety and easy concurrency the language offers. 

Immutable Variable Syntax Declartion:

`let x = 5;`

In case the programmer needs greater flexibility, Rust offers a mechanism to declare mutable variables.

Mutable Variable Syntax Declaration:

`let mut x = 5;`


- Control Flow
- Functions and closures
- Collections and User-Defined Types
    - Structures
    - Records
    - Classes
    - Enumerations: (often called an `Enum`) A Type that can be in one of multiple possible states.
        - Each possible state is a variant. 
    - Algebraic Data Types
- Abstraction and Polymorphism
- Errors and Missing Values
    - How a caller is expected to respond to an operation that can fail
        - Handling potential failure with `Result`
            - Result's variants are `ok` and `Err` 
            - An instance of Result has an expect method that you can call.

## Rust's Distinctive Features

- ownership and borrowing
- lifetimes conceptually
- traits and pattern matching
- copying and cloning
- safe concurrency

--

## Programming Evidence

- Focused Feature Examples
- Common Comparison Program
- Rust-Strength Program

---

## Comparative Discussion

- safety
- expressiveness
- learning difficulty
- performance/resource control
- development experience

## Limitations, and Conclusion

---

## References and AI Use Notes