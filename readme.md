# RC4 Variants PRNG Shootout
Benchmark performance of RC4 and its variants.

To run one of the binaries:

    $ cargo run --release --bin rc4a

Benchmarks on my Intel Core i7-8650U CPU @ 1.90GHz:

| PRNG   | MBps   |
|--------|:------:|
| RC4    | 429.13 |
| RC4A   | 679.78 |
| RC4+   | 430.68 |
| VMPC   | 327.31 |
| Spritz |  79.79 |
