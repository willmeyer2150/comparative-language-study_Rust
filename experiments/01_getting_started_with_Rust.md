# Getting Started with Rust

Experiment ID: E01
Date added: 2026-11-22 
Status: Complete

**Question:** What are you trying to find out?

How do you install Rust on our macOS and begin building programs?

**Prediction:** What do you expect, and why?

I expect there to be a standard method of installation that is curated by the Rust team to be simple and provide default functionality for anyone to begin coding in Rust. I also expect there to be custom installations for specific needs which are unlikely to be necessary for my current level. Finally, there will probably be cross-platform installation options for macOS, Windows and perhaps Linux.

**Setup:** Rust version, operating system, editor or terminal, and any relevant project files.

- Rust's current stable version is 1.100.0. 
- Current operating system: macOS Sequoia 15.7.4
- Default installation tool: `rustup`
- Installed directly through the terminal.

**Procedure:** Numbered steps someone else could repeat.

If on macOS, the following steps will install Rust on your system.

1. Go to https://rust-lang.org/tools/install/
2. Open a terminal on your mac.
3. Copy the following command into your command line:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

4. After Installation succeeds, set the path by entering the following command:

`. "$HOME/.cargo/env"`

5. Verify installation by entering the following two commands:
   
```
rustc --version
cargo --version
```

Rust is now installed and you are ready to write programs.

**Evidence:** Commands, code, compiler output, runtime output, timing, or screenshots.

Installation command: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
Set the path: `. "$HOME/.cargo/env"`
Verify installation: 

```
rustc --version
cargo --version
```

**Observation:** What happened, stated separately from interpretation.

After entering the installation command, a message was printed that Rust installed successfully. It was not clear to me that I needed to set the path for Cargo to operate properly. Cargo is Rust's build and package tool. It compiles code, runs it and manages dependencies declared in `Cargo.toml`

**Conclusion:** What does the evidence suggest about your question?

Installation is fairly straightforward and resources to help understand the process are available at Rust's main website: 

https://rust-lang.org

If you have additional installation questions, most of them can be quickly answered at their repository of supplementary documentation which is quite useful if new to Rust. It can be found at the following link:

https://forge.rust-lang.org/index.html#rust-forge

**Next step:** What small follow-up would resolve an uncertainty or extend the result?

The next step is to open vsCode and write a first program to experiment with the standard work flow in Rust.