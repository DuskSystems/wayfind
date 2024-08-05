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

## Parameters types

- Static: `/index.html`
- Dynamic Segment: `/users/<id>`
- Dynamic Inline: `/calender/<year>-<month>-<day>`
- Wildcard Segment(s): `/v1/<namespace:*>/tags/list`
- Wildcard End: `/<catch_all:*>`

### via `regex` feature

- Regex Segment: `/ids/<id:[0-9]+>`
- Regex Inline: `/user-<user_id:[0-9]{8}-[0-9]{4}-[0-9]{4}-[0-9]{4}>`

## Benchmarks

All benchmarks ran on a MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind) for a more accurate set of timings.

### [`matchit` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

```
matchit benchmarks/wayfind
  time: [224.82 ns 225.24 ns 225.73 ns]

matchit benchmarks/actix-router
  time: [20.848 µs 20.926 µs 21.011 µs]

matchit benchmarks/gonzales
  time: [129.74 ns 130.53 ns 131.51 ns]

matchit benchmarks/matchit
  time: [180.96 ns 181.28 ns 181.63 ns]

matchit benchmarks/ntex-router
  time: [1.5466 µs 1.5509 µs 1.5556 µs]

matchit benchmarks/path-table
  time: [545.95 ns 547.23 ns 548.67 ns]

matchit benchmarks/path-tree
  time: [326.97 ns 327.64 ns 328.39 ns]

matchit benchmarks/regex
  time: [1.1433 µs 1.1454 µs 1.1477 µs]

matchit benchmarks/route-recognizer
  time: [4.2956 µs 4.3079 µs 4.3215 µs]

matchit benchmarks/routefinder
  time: [6.6991 µs 6.7098 µs 6.7216 µs]

matchit benchmarks/xitca-router
  time: [353.24 ns 354.21 ns 355.28 ns]

matchit allocations
├─ wayfind           alloc:
│                      657
│                      125.1 KB
│                    dealloc:
│                      657
│                      153.7 KB
│                    grow:
│                      80
│                      28.64 KB
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
  time: [4.2992 µs 4.3121 µs 4.3265 µs]

path-tree benchmarks/actix-router
  time: [173.95 µs 174.45 µs 175.01 µs]

path-tree benchmarks/gonzales
  time: [5.7993 µs 5.8255 µs 5.8549 µs]

path-tree benchmarks/matchit
  time: [4.8306 µs 4.8723 µs 4.9377 µs]

path-tree benchmarks/ntex-router
  time: [26.698 µs 26.744 µs 26.801 µs]

path-tree benchmarks/path-table
  time: [10.303 µs 10.892 µs 11.641 µs]

path-tree benchmarks/path-tree
  time: [5.5977 µs 5.6225 µs 5.6498 µs]

path-tree benchmarks/regex
  time: [41.449 µs 42.091 µs 43.003 µs]

path-tree benchmarks/route-recognizer
  time: [84.758 µs 84.879 µs 85.021 µs]

path-tree benchmarks/routefinder
  time: [95.339 µs 95.577 µs 95.841 µs]

path-tree benchmarks/xitca-router
  time: [7.3345 µs 7.3524 µs 7.3706 µs]

path-tree allocations
├─ wayfind           alloc:
│                      1677
│                      334.9 KB
│                    dealloc:
│                      1677
│                      398.2 KB
│                    grow:
│                      195
│                      63.21 KB
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
