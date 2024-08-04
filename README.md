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

### Implemented

- Static: `/index.html`
- Dynamic Segment: `/users/{id}`
- Dynamic Inline: `/calender/{year}-{month}-{day}`
- Wildcard Segment(s): `/v1/{namespace:*}/tags/list`
- Wildcard End: `/{catch_all:*}`

### Planned

- Regex Segment: `/ids/{id:[0-9]+}`
- Regex Inline: `/user-{user_id:[0-9]{8}-[0-9]{4}-[0-9]{4}-[0-9]{4}}`

## Potential additional features

- Case insensitive matches
- URL encoding/decoding
- OpenAPI compatibility
- Routing via query parameters, host, methods, ...

## Benchmarks

All benchmarks ran on a MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind) for a more accurate set of timings.

### [`matchit` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

```
matchit benchmarks/wayfind
  time: [222.05 ns 222.63 ns 223.24 ns]

matchit benchmarks/actix-router
  time: [20.607 µs 20.670 µs 20.739 µs]

matchit benchmarks/gonzales
  time: [129.15 ns 129.43 ns 129.74 ns]

matchit benchmarks/matchit
  time: [180.97 ns 181.29 ns 181.66 ns]

matchit benchmarks/ntex-router
  time: [1.5582 µs 1.5616 µs 1.5658 µs]

matchit benchmarks/path-table
  time: [531.32 ns 532.15 ns 533.11 ns]

matchit benchmarks/path-tree
  time: [314.40 ns 315.52 ns 316.88 ns]

matchit benchmarks/regex
  time: [1.1640 µs 1.1684 µs 1.1733 µs]

matchit benchmarks/route-recognizer
  time: [4.3070 µs 4.3203 µs 4.3331 µs]

matchit benchmarks/routefinder
  time: [6.5039 µs 6.5201 µs 6.5386 µs]

matchit allocations
├─ wayfind           alloc:
│                      657
│                      88.37 KB
│                    dealloc:
│                      657
│                      108.3 KB
│                    grow:
│                      80
│                      19.93 KB
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
╰─ routefinder       alloc:
                       322
                       37.74 KB
                     dealloc:
                       322
                       58.74 KB
                     grow:
                       131
                       20.99 KB
```

### [`path-tree` benches](https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs)

```
path-tree benchmarks/wayfind
  time: [4.1107 µs 4.1179 µs 4.1259 µs]

path-tree benchmarks/actix-router
  time: [172.34 µs 172.79 µs 173.28 µs]

path-tree benchmarks/gonzales
  time: [5.7960 µs 5.8354 µs 5.8747 µs]

path-tree benchmarks/matchit
  time: [4.7748 µs 4.7830 µs 4.7923 µs]

path-tree benchmarks/ntex-router
  time: [26.684 µs 26.731 µs 26.787 µs]

path-tree benchmarks/path-table
  time: [10.210 µs 10.231 µs 10.257 µs]

path-tree benchmarks/path-tree
  time: [5.2184 µs 5.2325 µs 5.2493 µs]

path-tree benchmarks/regex
  time: [41.295 µs 41.461 µs 41.653 µs]

path-tree benchmarks/route-recognizer
  time: [85.154 µs 85.424 µs 85.700 µs]

path-tree benchmarks/routefinder
  time: [96.915 µs 97.096 µs 97.309 µs]

path-tree allocations
├─ wayfind           alloc:
│                      1677
│                      238.4 KB
│                    dealloc:
│                      1677
│                      282.6 KB
│                    grow:
│                      195
│                      44.2 KB
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
╰─ routefinder       alloc:
                       1131
                       115.8 KB
                     dealloc:
                       1131
                       179.3 KB
                     grow:
                       375
                       63.48 KB
```

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
