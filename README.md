# Conways Game of Life

## Background

From the
[Wikipedia](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) page:

> The Game of Life, also known simply as Life, is a cellular automaton
> devised by the British mathematician John Horton Conway in 1970.
>
> The game is a zero-player game, meaning that its evolution is
> determined by its initial state, requiring no further input. One
> interacts with the Game of Life by creating an initial configuration
> and observing how it evolves.

In this implementation, the initial state is decided randomly, by
providing a probability that any given cell will be alive.

## Code

I started this project because I wanted to learn the basics
of Rust. The main logic of the game is stored in
`src/lib.rs`, and is encapsulated by the `Board` structure.  The
source code for the two executables are kept in the
`src/bin/` directory. These two executables are:

- `visualiser`: A visualiser, useful for understanding
  the logic of the game and checking it runs as it should.
- `runner`: Runs the game repeatedly, saving the results in
  order to study the average behaviour at different starting
  fractions.

The output of `runner` is processed and analysed in
`analysis/plot_data.ipynb`.

## Future Work

* Produce interactive bokeh plot of data
* Finish writing analysis in `analysis.md`
* Merge `analysis.md` with `README.md`
* Pass data via reference from child to main thread for potential speedup and learning about lifetimes in Rust
* Write tests
