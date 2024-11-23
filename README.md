
# WASM_CHESS

Chess, but with WASM and the unicode chess characters

****

You can read more at [my website](https://sophie.coffee/fun/unicode-chess/)!

♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ Multiplayer Chess with WASM!  
♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟  
♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙  
♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖  

## The Plan

To make this semi-multiplayer!  
There will hopefully eventually be a *share* button that creates a link with the current game-state encoded in the URL queries as movement notation: `♙F2f4`.

## What I'm Using

* [`wasm_bindgen`](https://docs.rs/wasm-bindgen/) *Making everything WASM work*  
* [`web_sys`](https://docs.rs/web-sys/) *API bindings for making JavaScript calls from Rust*  

## Resources and Stuff

* If you want this, but smaller and already done - then [Tolido Nano-Chess](https://nanochess.org/chess4.html) exists!  
