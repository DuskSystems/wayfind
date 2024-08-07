[![Crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![Documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)

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

## Parameters

- Static: `/index.html`
- Dynamic: `/calender/<year>-<month>-<day>`
- Wildcard: `/v1/<namespace:*>/tags/list`

## Constraints

- Regex: `[a-z]+`
- Function: `is_lowercase_alpha`

## Benchmarks

All benchmarks ran on a MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind) for a more accurate set of timings.

### [`matchit` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

```
matchit benchmarks/wayfind
  time: [177.92 ns 178.21 ns 178.58 ns]

matchit benchmarks/actix-router
  time: [20.811 µs 21.040 µs 21.435 µs]

matchit benchmarks/gonzales
  time: [128.86 ns 128.98 ns 129.15 ns]

matchit benchmarks/matchit
  time: [180.75 ns 180.95 ns 181.21 ns]

matchit benchmarks/ntex-router
  time: [1.5571 µs 1.5602 µs 1.5640 µs]

matchit benchmarks/path-table
  time: [532.16 ns 533.23 ns 534.50 ns]

matchit benchmarks/path-tree
  time: [327.58 ns 328.18 ns 328.91 ns]

matchit benchmarks/regex
  time: [1.1405 µs 1.1442 µs 1.1481 µs]

matchit benchmarks/route-recognizer
  time: [4.3165 µs 4.3230 µs 4.3299 µs]

matchit benchmarks/routefinder
  time: [6.2712 µs 6.2829 µs 6.2969 µs]

matchit benchmarks/xitca-router
  time: [355.19 ns 355.78 ns 356.46 ns]

matchit allocations
├─ wayfind           alloc:
│                      654
│                      100.9 KB
│                    dealloc:
│                      654
│                      100.9 KB
│                    grow:
│                      80
│                      19.74 KB
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

```
path-tree benchmarks/wayfind
  time: [3.0148 µs 3.0200 µs 3.0256 µs]

path-tree benchmarks/actix-router
  time: [173.24 µs 173.70 µs 174.22 µs]

path-tree benchmarks/gonzales
  time: [5.8082 µs 5.8474 µs 5.8877 µs]

path-tree benchmarks/matchit
  time: [4.8001 µs 4.8139 µs 4.8322 µs]

path-tree benchmarks/ntex-router
  time: [27.171 µs 27.204 µs 27.245 µs]

path-tree benchmarks/path-table
  time: [10.323 µs 10.345 µs 10.371 µs]

path-tree benchmarks/path-tree
  time: [5.5324 µs 5.5452 µs 5.5584 µs]

path-tree benchmarks/regex
  time: [41.077 µs 41.219 µs 41.389 µs]

path-tree benchmarks/route-recognizer
  time: [85.506 µs 85.607 µs 85.733 µs]

path-tree benchmarks/routefinder
  time: [91.992 µs 92.100 µs 92.234 µs]

path-tree benchmarks/xitca-router
  time: [7.2957 µs 7.3110 µs 7.3296 µs]

path-tree allocations
├─ wayfind           alloc:
│                      1619
│                      268.1 KB
│                    dealloc:
│                      1619
│                      308.7 KB
│                    grow:
│                      195
│                      40.65 KB
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
