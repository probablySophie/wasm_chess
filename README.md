
# WASM/Unicode Chess
<img align="right" src="./.meta/Logo.svg" align="left" height="128" width="128" alt="Logo">
Chess, but with WASM and the [unicode chess characters](https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode)!


You can read more at [my website](https://sophie.coffee/fun/unicode-chess/) and there's a [live version](https://probablysophie.github.io/wasm_chess/) you can play!

♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ Multiplayer  
♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ Chess  
♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ with  
♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ WASM!  

## The Plan

**To make this semi-multiplayer!**  
> There will hopefully eventually be a *share* button that creates a link with the current game-state encoded in the URL queries as movement notation: `♙F2f4`.

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




## The Todo

- [x] WASM connect in & show the pieces
- [x] Make it possible to move pieces
- [ ] Move validation
- [x] GitHub pages for it pls *(statically host the `www/index.html`?) (and link it here)*
- [ ] Make it possible to *take* pieces  
- [ ] https://en.wikipedia.org/wiki/Rules_of_chess  
- [ ] Semi-Multiplayer with URL encoded [move histories](https://en.wikipedia.org/wiki/Chess_notation) for sharing 
  * Long algebraic probably  
- [ ] Show the move histories from the URL  


## Future Fancy Features

- [ ] An option to undo the move you just did (aimed at mobile misclicks)
- [ ] Confirmation if someone wins
- [ ] If in check, all moves that don't get you out of check are invalid
  - [ ] Visual highlighting that you're in check?
- [ ] Make sure the text line-height and table row-height are *exactly* the same (they currently aren't)
- [ ] Add async so we can make a 200ms per move recap of where we are?
  * Or figure out how to use `setInterval` from WASM?
- [ ] A little red flash or something when an invalid move is attempted?  
- [ ] Little ghosts showing all valid moves (with arrows for valid in-between moves)  
- [ ] Flip the horsey on the left so it looks prettier  
  * This would require tracking the left horsey down from all it's previous moves
- [ ] Something subtle to mark who's turn it is?
- [ ] A share button that uses [navigator.share()](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share) from the [Web Share API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Share_API) *(if available)*
