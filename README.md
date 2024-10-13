# My algorithm in Rust
This is a private project.


## Notes
Install https://code.visualstudio.com/docs/languages/rust

## TODOs
- [x] Protect best node from pruning
- [x] Redo pruning on max_pos after change
- [x] Node collapsing: avoid node repeating "inline". Instead of check recursively like done in python I can touch/verify the element on that the tree pruning stops. To be checked also the new nodes when I'm leaving them.
- [x] Tree mode
- [ ] Extensive tests on what I did until now
- [ ] Update thesis wrt updates in code
- [ ] Non-branching nodes: move them in a dedicated structure property?

## "Benchmark"

Matrix size 2_885 x 23_550 = 67_941_750

Windows, not optimized compilation:
Tree size  | Versione   | RAM (approx)
----------:|:-----------|-------------
67_941_750 | 1 (full)   | ~11gb (win, non opt)
119_072 | 2 (pruning)   | ~130mb (win, non opt)
18_658 | 3 (collapsing) | ~35mb (win, non opt)
4_886 | 4 (tree mode)   | ~19mb (win, non opt)

Linux, optimized compilation:
Tree size  | Versione   | RAM (approx)            | time
----------:|:-----------|-------------------------|-----
67_941_750 | 1 (full)   | 10_998_636kb (lin, opt) | 46.83s
119_072 | 2 (pruning)   | 51556kb (lin, opt)      | 34.33s
18_658 | 3 (collapsing) | 6684kb (lin, opt)       | 27.17s
4_886 | 4 (tree mode)   | 6124kb (lin, opt)       | 24.85s

Note 1: RAM in the first table is measured as approximation during non-optimized execution with cargo.

Note 2: m is 2_884 and m^2 is 8_317_456. n+m is 26_433

Note 3: Alignment Score is 1_431 and max_pos is 15_919_429



====== ONLY TREE

        Command being timed: "target/release/thesis_rust"
        User time (seconds): 24.85
        System time (seconds): 0.12
        Percent of CPU this job got: 99%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:25.20
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 6124
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 601
        Voluntary context switches: 74
        Involuntary context switches: 121
        Swaps: 0
        File system inputs: 0
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0

== FULL PATH, FULL OPTIMIZATION

        Command being timed: "target/release/thesis_rust"
        User time (seconds): 27.17
        System time (seconds): 0.10
        Percent of CPU this job got: 99%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:27.55
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 6684
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 804
        Voluntary context switches: 69
        Involuntary context switches: 207
        Swaps: 0
        File system inputs: 0
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0

=========== FULL PATH, NO COLLAPSING

        Command being timed: "target/release/thesis_rust"
        User time (seconds): 34.33
        System time (seconds): 0.18
        Percent of CPU this job got: 98%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:35.14
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 51556
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 2763
        Voluntary context switches: 136
        Involuntary context switches: 1661
        Swaps: 0
        File system inputs: 0
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0

======= NO PRUNING

        Command being timed: "target/release/thesis_rust"
        User time (seconds): 46.83
        System time (seconds): 34.69
        Percent of CPU this job got: 84%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 1:36.87
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 10998636
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 345555
        Minor (reclaiming a frame) page faults: 4132446
        Voluntary context switches: 230942
        Involuntary context switches: 840
        Swaps: 0
        File system inputs: 1885648
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0