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

Tree size  | Versione
----------:|:--------
67_941_750 | 1 (full)
119_072 | 2 (pruning)
18_658 | 3 (collapsing)
4_886 | 4 (tree mode)

m is 2_884 and m^2 is 8_317_456. n+m is 26_433

Alignment Score is 1_431 and max_pos is 15_919_429





=======================

Matrix size 2885 x 23550 = 67941750
Tree size 18658 nodes (0% of matrix size)
m is 2884 and m^2 is 8317456. n+m is 26433

Full schema saved in memory too big to be printed, sorry

Path from best score to root (w=15919429)
Alignment Score: 1431
max_pos: 15919429