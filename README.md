# 100 Doors
> Imagine a hundred closed doors in a corridor.
> You go and open every one of them.
> Then, you open every second one. (If the door was open, then just close it.)
> Then, every third. Then, every fourth and every fifth and so on until you only touch the 100th door.
> How many doors are open now?

This is a mathematical problem known as "100 Doors". I have written a smol
program in Rust that calculates and visualizes the solution using
[ncurses-rs](https://crates.io/crates/ncurses).

```
A simple program that calculates and visualizes the "100 Doors" problem

Usage: hundred-doors [OPTIONS] [NUMBER]

Arguments:
  [NUMBER]  How many doors there are [default: 100]

Options:
  -d, --draw           Whether there should be a graphical representation of these doors
  -e, --explain        Whether to explain the problem
      --twave <TWAVE>  How many milliseconds to wait before the next wave [default: 0]
      --tdoor <TDOOR>  How many milliseconds to wait between new doors [default: 0]
  -h, --help           Print help
  -V, --version        Print version
```

## Examples
```
$ hundred-doors 100
> 10
```

```
$ hundred-doors 100 --draw --tdoor 10 --twave 50
```

<center>*ncurses opens*</center>