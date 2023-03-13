# Welcome to My Rust-venture!


### What is this repo?
This repo is my learning space for the Rust programming language.  Not too much to say, 

----

### How to get this set up...
Windows PC:
- Download the 64-bit installer from [this link](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
- Double check that the installation was successful by running the following commands from your terminal of choice:
  - `rustup update`
  - `cargo --version`

MacOS:
- TBD, don't have a Mac currently available or configured for development

Linux:
- TBD, I'm sure it has distro-specific details available

----

### Once things are ready...
Make sure you're in the main directory for this whole repo, `\rusting_away`.  Once you are, you can make a new project
by running the command `cargo new {my-new-project-name}`.  This will create the bare-bones framework for a new project.
Once you've gotten the code you want to run into the main file, use the command `cargo run` to run it!

You can also leverage the rustfmt utility similar to other code fomatting tools (such as black for Python).
To do so, you can run `rustfmt {my-filename-to-format}`.  If you provide a file path in place of a file name, you will
receive an error.  To run `rustfmt` on a directory, navigate the terminal to the directory of interest containing the
Rust code to format and run just `rustfmt`.  This version may take some time, so it is currently recommended to run on
a by-file basis as a pre-commit-like check.

If you wanted to compile and run the code yourself, you can run the command `rustc {my-rust-file}`.  You can then run 
the generated .exe file from the same terminal instance.