# Advent of Code 2024

This repo is my collection of solutions for the Advent of Code 2024!

Each solution is done in either Rust or Nushell or both.

## Running solutions

Make sure you put all your inputs inside `./inputs/day{number}.txt`.

Then for rust simply run `cargo run -- {number}` to run the given number.

All nushell solutions are in the `./nu/{day{number}.nu}` files and can be ran
using `nu ./nu/day{number}.nu` for your particular day.

## Why Rust and Nushell

Rust is the main programming language I use at the moment and this is a good
excuse to sharpen my skills at Rust and work with Rust applications.

Nushell is a bit of a weirder choice however I found that while daily driving
Nushell as default shell, I found I am using it more and more to solve quick
scripting-like tasks that I would otherwise use Python for.

Mainly because I like it's functional like programming concepts using higher
order functions, and it has great out-of-the-box support for YAML, JSON, XML,
CSV, regex parsing and more.

Although some of this year's problems are starting to get fairly convoluted, I'm
surprised to see that some problems are 1 simple Nushell command away from being
solved.

Overall, using both of these languages to solve each day's problems challenges
me to solve the problem using more of a multi-paradigm approach with Rust and a
functional paradigm with Nushell which will come in handy when working in Ocaml,
Elm or Haskell.

## Completed Days

Here is the table of completed days,
✅ = done, ❌ = currently not done or failed.

Each cell has 2 spots indicating part 1 and part 2 respectively.

| Day | Rust | Nushell |
| --- | ---- | ------- |
| 1   | ✅✅ | ✅❌    |
| 2   | ✅❌ | ✅✅    |
| 3   | ✅✅ | ✅✅    |




