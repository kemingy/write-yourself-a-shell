# rush - Build your own shell in Rust!

One of things developers use every day is a shell. It comes in many flavors like bash, fish or zsh. Depending on your background, writing your own shell from scratch might either sound intimidating or pointless.
We like to believe that it can be a fun way to gain more hands-on Rust experience.
If you want to play with concepts like I/O, error handling, and syscalls we invite you to participate in this workshop. Who knows, it could also be an opportunity to start your first mid-size Rust project?

## Just tell me what I'll learn

After this workshop you will know how to 
- Show a prompt to the user
- Read command-line input
- Execute commands
- Print output
- Pipe all the things

## Who's the target audience?

This workshop is intended for *intermediate* Rust programmers that already understand basic Rust concepts and have some programming experience. We’ll explain the rest as we go along.
When we were learning Rust and already knew the basic concepts, we always wanted to see a mid-size system being built to get an understanding for how the concepts are put into practice. We think this could be a learning boost for people that are in the same situation like we were.

[Santiago Pastorino](https://santiagopastorino.com) is the [WyeWorks](https://wyeworks.com) co-founder, Ruby on Rails core team alumni and Rust Developer. He is currently part of the Rust compiler NLL WG and [Rust Latam](https://rustlatam.org) organizer.
[Matthias Endler](https://matthias-endler.de ) is a Backend Engineer at trivago and runs [Hello Rust Show](https://hello-rust.show).

## Introduction

- About Santiago and Matthias
- We are just humans. We will make mistakes
- Why we like Rust
- Why the workshop
- Why a shell?
  * A tool we use every day
  * Touches on many subjects (cli, error handling,...)
  * Fun to write an to extend
  * It's cool.
- Scope of the workshop
- Disclaimer: This is a toy shell and not production-ready, but rather an excuse to learn Rust.
  A shell is a really big project. We can not cover everything.  
- It's probably not the correct way to implement the features as we do. Rather a way that makes sense for a workshop.

We look forward to your own ideas.

## Organizational notes

### Time

This is a six hour workshop, split into six blocks of one hour each.

Each block consists of...

* 30min hacking time
* 30min reflection on what we did plus recovery

###  Structure

- You can always come and go.
- Before every block, we will share a `git checkout <commit_id>` command with
  the code for every block.
- Always ask for help.
- Pair programming is encouraged.
- Skip ahead if you like.
- We will split up and provide support.

### Necessary tools 
    
* rustc
* cargo
* git

## What is a shell?

### History of shells

* https://en.wikipedia.org/wiki/Unix_shell
* https://multicians.org/shell.html

### Terminology

1. What is a process?
2. `stdout` and `stderr`
4. pipes are amazing
5. Output redirection

### Modern shell features

1. Running single commands
2. Concatenating commands with semicolon
3. Shell-builtins (cd)
4. Pipes
5. Ideas for extending your shell
  - readline
  - handling control signals (ctrl+c, ctrl+d)  
  - redirection
  - command completion
  - adding more builtins
  - use a grammar for parsing
  - Hints for the workshop

### Section 0 - Check Rust installation and version

- rustc -v
- try running Rust Edition 2018 preview

### Section 1 - Getting up and running

- Try to run a single command and print the output to the stdout.
- Hint: Look for APIs in the standard library to do that.
- Bonus: Try to make the code as idiomatic as you can.
- Bonus: Write a unit test to make sure your shell works

### Section 2 - Concatenating commands

- Try to run two commands in sequence and print all output in sequence
  to stdout
- Bonus: Write a functional test (e.g.check out
  https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html)

### Section 3 - Builtins

- What would be a nice way to handle builtins?
- Implement a builtin of your choice (e.g. exit, echo, cd, history,
  for,...)
- Bonus: Think about why \`cd\` must be a builtin

### Section 4 - Pipes
### Section 5 - Bring your own features! (Where the fun is at)

- Readline
- Control signals
- redirection
- command completion
- use a grammar for parsing
- Implement more shell-builtins
- Add more tests

