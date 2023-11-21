# Rustsidian-todo
Just a mix of obsidian and Rust for tracking todo-lists in your favorite obsidian vaults.
## Requirements
You need to have Rust installed. If you don't, you can install it from [here](https://www.rust-lang.org/tools/install).
## Usage
First of all, let's install the program:
```bash
git clone https://www.github.com/sellsword9/rustsidian-todo
```
Then, we need to build it:
```bash
cd rustsidian-todo && cargo build --release
```
Now we just need to create or move a Obsidian vault to the same directory as the executable and run it.
By default, the program expects the vault to be named 'head' and the todo file to be named 'TODO.md'.
Changing that is very easy. Just use your favorite text/code editor and open the file 'src/main.rs'.
Then, change the values of the variable named PATH
It should be in the third line and look like this:
```rust
const PATH: &str = "head/TODO.md";
```
After that, just build the program again: 
```bash
cargo build --release
```
And you're done! Now you can run the program and it will use the new path.

# Want a faster installation?
```bash
git clone https://www.github.com/sellsword9/rustsidian-todo && cd rustsidian-todo && cargo build --release && cd src; vim main.rs;
```
Then, change the path. If you don't have vim, you can use nano, or vsc, or nvim... After you're done, just run this to build
```bash
cargo build --release
```
And you're done. Now you can run this to get help on usage in your own terminal
```bash
cargo r -- -h
```

# Help message

Running with cargo or no arguments will use -n
Usage: 
    -n Basic output, outputs the number of unchecked tasks
    -t Outputs the number of tasks, done or not
    -d Outputs the number of done tasks
    -h Shows this help message
