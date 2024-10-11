# My algorithm in Rust
This is a private project.


## Notes
Install https://code.visualstudio.com/docs/languages/rust

## TODOs
- [x] Protect best node from pruning
- [x] Redo pruning on max_pos after change
- [ ] Extensive tests on this part before continue
- [ ] Update thesis wrt updates in code

- [ ] Node collapsing: avoid node repeating "inline". Instead of check recursively like done in python I can touch/verify the element on that the tree pruning stops. To be checked also the new nodes when I'm leaving them.
- [ ] Non-branching nodes: move them in a dedicated structure property

## "Benchmark"

Matrix size 2_885 x 23_550 = 67_941_750

Tree size  | Versione   | RAM (approx)
----------:|:-----------|-------------
67_941_750 | 1 (full)   | 11gb
119_072 | 2 (pruning)   | 130mb
18_658 | 3 (collapsing) | 35mb
4_886 | 4 (tree mode)   | 19mb

Note 1: RAM is measured as approximation during non-optimized execution with cargo.

Note 2: m is 2_884 and m^2 is 8_317_456. n+m is 26_433

Note 3: Alignment Score is 1_431 and max_pos is 15_919_429
