# Remember Code

[Licensed under the MIT license.](LICENSE.MIT)

A simple GUI tool for finding out what kind of simple memory rules there are for a six-digit sequences.

It is based on a list of rules it iterates for the sequence.

The default function is to find out how many sequences in total match any of the rules—in other words, how likely it is for a six-digit sequence to be "easy"—but it can also display the rules found for the specified sequence.

You can find the rules supported from [src/rules.rs](src/rules.rs). They basically boil down to:

- a sequence made from adjacent digits in the main directions ("worm") skipping 0 or 1 digits, with wrap around from the number pad edges
- a sequence made from adjacent digits in the diagonal axis ("diagonal worm") in a number pad
- a sequence that follows an arithmetic sequence, e.g. 1, 2, 3 or 4, 6, 7

Most all number pad shapes are considered for the rules involving one.

# Installing

* [Install Rust](https://rustup.rs/)
* cargo install --git https://github.com/eras/reco

# Running

```
% reco
```
