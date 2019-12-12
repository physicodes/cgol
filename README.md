# Conways Game of Life

## Background

From the Wikipedia page:

> The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970.
>
> The game is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. One interacts with the Game of Life by creating an initial configuration and observing how it evolves.

In this implementation, the initial state is decided randomly, by providing a probability that any given cell will be alive.

## Code

I started this project because I wanted to learn the basics of Rust. All the Rust code is included in a single file `src/main.rs`. Also in the top level directory is `demo.py`, a quick and dirty implementation in Python which I wrote to ensure I was clear on the game's logic before starting out in Rust.

In the future I may extend this code to include benchmarks of the Rust vs Python version, and also a Monte Carlo simulation to investigate how the total population changes over many iterations (i.e. whether a minimum is reached, how the starting fraction affects this evolution).
