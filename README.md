# Chess Rust

Yet another chess engine. I am just putting this repository on public for other people to see, some of the techniques used here are related to other chess engines, there's nothing new or suprising about this engine.

This is an improved version of this [old chess engine](https://github.com/DestinEcarma/chess-rust-old) I made.

## Inspiration

The development of this project is heavily inspired by [Rustic](https://github.com/mvanthoor/rustic). The techniques and methodologies employed here are largely derived from this source.

## Getting Started

To start using the engine simply type this command:

```
cargo run
```

There's really nothing else you can do here other than play against your self or do a perfromance test:

```
perft <depth>		Example: perft 7
move <move>			Example: move d2d3
fen <fen string>	Example fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -
undo				simply undo the move
display				display the current board
```