
# WASM_CHESS

Chess, but with WASM and the unicode chess characters

****

You can read more at [my website](https://sophie.coffee/fun/unicode-chess/)!

♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ Multiplayer Chess with WASM!  
♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟  
♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙  
♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖  

## The Plan

**To make this semi-multiplayer!**  
> There will hopefully eventually be a *share* button that creates a link with the current game-state encoded in the URL queries as movement notation: `♙F2f4`.

**To make GitHub host this for me**
> I just need to make a GitHub action that statically hosts the `www/` folder

**To make this cute**  
> I want this to be minimal and just kind of nice looking, I think I'm doing a decent job so far.  
> I'm planning on adding a little move queue to the right, and a little recap animation when you load the page (if there is a query list)

## What I'm Using

* [`wasm_bindgen`](https://docs.rs/wasm-bindgen/) *Making everything WASM work*  
* [`web_sys`](https://docs.rs/web-sys/) *API bindings for making JavaScript calls from Rust*  
* [Catppuccin colours!](https://catppuccin.com/palette)  

## Local Testing

You'll need all the WASM tools:  
```bash
rustup target add wasm32-unknown-unknown # This installs almost instantly
# This set of installs *may* slow down your computer for a little while
cargo install wasm-bindgen-cli
cargo install wasm-gc
cargo install wasm-opt
```

Then just `. run.sh` (if you're on Linux...)  

## Resources and Stuff

* If you want this, but smaller and already done - then [Tolido Nano-Chess](https://nanochess.org/chess4.html) exists!  
