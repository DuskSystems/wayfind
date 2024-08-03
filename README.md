[![Crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![Documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)

# `wayfind`

A speedy, flexible router for Rust.

Currently in a pre-alpha state.

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
- Wildcard End: `/{catch_all:*}`

### Planned

- Wildcard Segment(s): `/v1/{namespace:*}/tags/list`
- Regex Segment: `/ids/{id:[0-9]+}`
- Regex Inline: `/user-{user_id:[0-9]{8}-[0-9]{4}-[0-9]{4}-[0-9]{4}}`

### Under consideration

- Wildcard Inline: `/files/{file:*}.{extension}`

Currently, inline dynamic variables are greedy in nature, but maybe we'd be better off using inline wildcards to allow choosing between greedy and lazy matching.

### Unlikely

Could be convinced to add these, but likely not in scope for `v1`.

- Optional: `/photos/{id:?}`
- Optional Groups: `/images/{name}(.{type})`

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
  time: [170.17 ns 170.52 ns 170.95 ns]

matchit benchmarks/actix-router
  time: [20.496 µs 20.557 µs 20.625 µs]

matchit benchmarks/gonzales
  time: [129.23 ns 129.42 ns 129.64 ns]

matchit benchmarks/matchit
  time: [179.99 ns 180.30 ns 180.67 ns]

matchit benchmarks/ntex-router
  time: [1.5541 µs 1.5569 µs 1.5603 µs]

matchit benchmarks/path-table
  time: [535.73 ns 539.85 ns 545.66 ns]

matchit benchmarks/path-tree
  time: [317.84 ns 318.79 ns 319.77 ns]

matchit benchmarks/regex
  time: [1.1426 µs 1.1454 µs 1.1484 µs]

matchit benchmarks/route-recognizer
  time: [4.2810 µs 4.2940 µs 4.3103 µs]

matchit benchmarks/routefinder
  time: [6.2914 µs 6.2995 µs 6.3092 µs]

matchit allocations
├─ wayfind           alloc:
│                      262
│                      66.97 KB
│                    dealloc:
│                      262
│                      84.48 KB
│                    grow:
│                      80
│                      17.5 KB
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
  time: [3.3377 µs 3.3423 µs 3.3476 µs]

path-tree benchmarks/actix-router
  time: [172.97 µs 173.34 µs 173.76 µs]

path-tree benchmarks/gonzales
  time: [5.8393 µs 5.8916 µs 5.9629 µs]

path-tree benchmarks/matchit
  time: [4.8119 µs 4.8198 µs 4.8285 µs]

path-tree benchmarks/ntex-router
  time: [26.801 µs 26.848 µs 26.907 µs]

path-tree benchmarks/path-table
  time: [10.195 µs 10.219 µs 10.248 µs]

path-tree benchmarks/path-tree
  time: [5.1718 µs 5.1810 µs 5.1918 µs]

path-tree benchmarks/regex
  time: [41.271 µs 41.425 µs 41.605 µs]

path-tree benchmarks/route-recognizer
  time: [86.283 µs 86.456 µs 86.681 µs]

path-tree benchmarks/routefinder
  time: [86.989 µs 87.113 µs 87.266 µs]

path-tree allocations
├─ wayfind           alloc:
│                      665
│                      174.5 KB
│                    dealloc:
│                      665
│                      214.3 KB
│                    grow:
│                      195
│                      39.79 KB
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
