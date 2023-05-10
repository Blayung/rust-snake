## What is it?
It is a snake game written in rust using the sdl2 library. I made it just to get familiar with rust and practice programming in it. You cannot go through walls and when the game is lost the program panics with a "GAME OVER!" message. You cannot see snake's directions. It's also a lot of terrible code.

### Compiling (assuming you're on linux)
0. Make sure you have git and cargo installed! If not, check these websites out: [https://git-scm.com/downloads](https://git-scm.com/downloads), [https://rustup.rs](https://rustup.rs)
1. `git clone https://github.com/Blayung/rust-snake`
2. `cd rust-snake`
3. `cargo build --release`
4. Now the executable should be here: `./target/release/snake`
