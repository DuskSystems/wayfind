[![Crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![Documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)
[![Codecov Badge](https://codecov.io/github/DuskSystems/wayfind/graph/badge.svg?token=QMSW55438K)](https://codecov.io/github/DuskSystems/wayfind)

# `wayfind`

A speedy, flexible router for Rust.

> [!WARNING]
> Currently in a pre-alpha state.

## Why another router?

`wayfind` tries to bridge the gap between existing Rust router options:

- fast routers, lacking in flexibility
- flexible routers, lacking in speed

Real-world projects often need fancy routing capabilities, such as projects ported from frameworks like [Ruby on Rails](https://guides.rubyonrails.org/routing.html), or those adhering to specifications like the [OpenContainers Distribution Spec](https://github.com/opencontainers/distribution-spec/blob/main/spec.md).

The goal of `wayfind` is to remain competitive with the fastest libraries, while offering advanced routing features when needed. Unused features shouldn't impact performance - you only pay for what you use.

## Benchmarks

All benchmarks ran on a MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind/benchmarks) for a more accurate set of timings.

### [`matchit` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

> In a router of 130 routes, match 4 paths.

```
matchit benchmarks/wayfind
  time: [205.06 ns 205.32 ns 205.66 ns]

matchit benchmarks/actix-router
  time: [21.087 µs 21.264 µs 21.440 µs]

matchit benchmarks/gonzales
  time: [129.58 ns 130.01 ns 130.50 ns]

matchit benchmarks/matchit
  time: [180.11 ns 180.46 ns 180.94 ns]

matchit benchmarks/ntex-router
  time: [1.5558 µs 1.5605 µs 1.5656 µs]

matchit benchmarks/path-table
  time: [539.10 ns 540.01 ns 541.06 ns]

matchit benchmarks/path-tree
  time: [312.92 ns 313.57 ns 314.35 ns]

matchit benchmarks/regex
  time: [1.1500 µs 1.1621 µs 1.1833 µs]

matchit benchmarks/route-recognizer
  time: [4.3255 µs 4.3450 µs 4.3664 µs]

matchit benchmarks/routefinder
  time: [6.0104 µs 6.0235 µs 6.0387 µs]

matchit benchmarks/xitca-router
  time: [352.04 ns 352.75 ns 353.61 ns]

matchit allocations
├─ wayfind           alloc:
│                      654
│                      97.43 KB
│                    dealloc:
│                      654
│                      116.5 KB
│                    grow:
│                      80
│                      19.13 KB
│
├─ actix-router      alloc:
│                      31187
│                      38.03 MB
│                    dealloc:
│                      30985
│                      41.25 MB
│                    grow:
│                      6502
│                      3.228 MB
│                    shrink:
│                      183
│                      526 B
│
├─ gonzales          alloc:
│                      1014
│                      5.426 MB
│                    dealloc:
│                      1014
│                      5.434 MB
│                    grow:
│                      203
│                      8.272 KB
│
├─ matchit           alloc:
│                      1106
│                      67.99 KB
│                    dealloc:
│                      1106
│                      89.93 KB
│                    grow:
│                      89
│                      21.93 KB
│
├─ ntex-router       alloc:
│                      36933
│                      71.99 MB
│                    dealloc:
│                      36731
│                      73.92 MB
│                    grow:
│                      6729
│                      1.93 MB
│                    shrink:
│                      6
│                      23 B
│
├─ path-table        alloc:
│                      406
│                      39.73 KB
│                    dealloc:
│                      406
│                      39.73 KB
│
├─ path-tree         alloc:
│                      968
│                      70.79 KB
│                    dealloc:
│                      968
│                      93.03 KB
│                    grow:
│                      74
│                      22.24 KB
│
├─ regex             alloc:
│                      17782
│                      1.824 MB
│                    dealloc:
│                      17782
│                      3.619 MB
│                    grow:
│                      2747
│                      1.8 MB
│                    shrink:
│                      215
│                      5.938 KB
│
├─ route-recognizer  alloc:
│                      1437
│                      60.11 KB
│                    dealloc:
│                      1437
│                      222.3 KB
│                    grow:
│                      75
│                      162.2 KB
│
├─ routefinder       alloc:
│                      322
│                      37.74 KB
│                    dealloc:
│                      322
│                      58.74 KB
│                    grow:
│                      131
│                      20.99 KB
│
╰─ xitca-router      alloc:
                       1311
                       55.61 KB
                     dealloc:
                       1311
                       73.49 KB
                     grow:
                       89
                       17.88 KB
```

### [`path-tree` benches](https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs)

> In a router of 320 routes, match 80 paths.

```
path-tree benchmarks/wayfind
  time: [3.1300 µs 3.1345 µs 3.1398 µs]

path-tree benchmarks/actix-router
  time: [175.08 µs 177.90 µs 183.39 µs]

path-tree benchmarks/gonzales
  time: [5.7293 µs 5.7671 µs 5.8043 µs]

path-tree benchmarks/matchit
  time: [4.8003 µs 4.8068 µs 4.8146 µs]

path-tree benchmarks/ntex-router
  time: [26.451 µs 26.509 µs 26.582 µs]

path-tree benchmarks/path-table
  time: [10.299 µs 10.323 µs 10.351 µs]

path-tree benchmarks/path-tree
  time: [5.2089 µs 5.2175 µs 5.2274 µs]

path-tree benchmarks/regex
  time: [41.535 µs 41.685 µs 41.874 µs]

path-tree benchmarks/route-recognizer
  time: [86.125 µs 86.327 µs 86.578 µs]

path-tree benchmarks/routefinder
  time: [91.673 µs 91.784 µs 91.906 µs]

path-tree benchmarks/xitca-router
  time: [7.3105 µs 7.3279 µs 7.3501 µs]

path-tree allocations
├─ wayfind           alloc:
│                      1619
│                      258.5 KB
│                    dealloc:
│                      1619
│                      298.1 KB
│                    grow:
│                      195
│                      39.55 KB
│
├─ actix-router      alloc:
│                      78390
│                      94.92 MB
│                    dealloc:
│                      77862
│                      104.7 MB
│                    grow:
│                      16411
│                      9.872 MB
│                    shrink:
│                      510
│                      1.856 KB
│
├─ gonzales          alloc:
│                      2638
│                      14.26 MB
│                    dealloc:
│                      2638
│                      14.28 MB
│                    grow:
│                      578
│                      19.53 KB
│
├─ matchit           alloc:
│                      2951
│                      204.3 KB
│                    dealloc:
│                      2951
│                      261.4 KB
│                    grow:
│                      218
│                      57.03 KB
│
├─ ntex-router       alloc:
│                      97922
│                      188.5 MB
│                    dealloc:
│                      97394
│                      193.5 MB
│                    grow:
│                      17688
│                      5.016 MB
│
├─ path-table        alloc:
│                      1192
│                      105.7 KB
│                    dealloc:
│                      1192
│                      105.7 KB
│
├─ path-tree         alloc:
│                      2471
│                      184.1 KB
│                    dealloc:
│                      2471
│                      233.5 KB
│                    grow:
│                      177
│                      49.44 KB
│
├─ regex             alloc:
│                      48316
│                      4.547 MB
│                    dealloc:
│                      48316
│                      8.485 MB
│                    grow:
│                      6982
│                      3.942 MB
│                    shrink:
│                      568
│                      4.694 KB
│
├─ route-recognizer  alloc:
│                      6152
│                      321.2 KB
│                    dealloc:
│                      6152
│                      981.4 KB
│                    grow:
│                      268
│                      660.1 KB
│
├─ routefinder       alloc:
│                      1131
│                      115.8 KB
│                    dealloc:
│                      1131
│                      179.3 KB
│                    grow:
│                      375
│                      63.48 KB
│
╰─ xitca-router      alloc:
                       3548
                       179.1 KB
                     dealloc:
                       3548
                       226.8 KB
                     grow:
                       218
                       47.65 KB
```

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
