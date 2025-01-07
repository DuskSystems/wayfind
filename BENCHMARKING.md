# Benchmarking

All benchmarks ran on a M1 Pro laptop running Asahi Linux.

Check out [codspeed](https://codspeed.io/DuskSystems/wayfind/benchmarks) for an additional set of results.

## Context

For all benchmarks, we convert any extracted parameters to strings.

All routers provide a way to return parameters as strings, but some delay the actual UTF-8 decoding until post-search.

| Library          | Percent Decoding | String Parameters |
|:-----------------|:----------------:|:-----------------:|
| wayfind          | no               | yes               |
| actix-router     | partial          | yes               |
| matchit          | no               | delayed           |
| ntex-router      | partial          | yes               |
| path-tree        | no               | delayed           |
| route-recognizer | no               | yes               |
| routefinder      | no               | yes               |
| xitca-router     | no               | yes               |

## `matchit` inspired benches

In a router of 130 templates, benchmark matching 130 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 7.7684 µs | 4           | 288 B      | 4             | 288 B        |
| matchit          | 11.555 µs | 4           | 448 B      | 4             | 448 B        |
| path-tree        | 12.479 µs | 4           | 448 B      | 4             | 448 B        |
| xitca-router     | 14.823 µs | 7           | 832 B      | 7             | 832 B        |
| ntex-router      | 51.490 µs | 18          | 1.28 KB    | 18            | 1.28 KB      |
| route-recognizer | 69.752 µs | 160         | 8.537 KB   | 160           | 8.537 KB     |
| routefinder      | 173.13 µs | 67          | 5.056 KB   | 67            | 5.056 KB     |
| actix-router     | 513.33 µs | 214         | 13.96 KB   | 214           | 13.96 KB     |

## `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 3.9816 µs | 59          | 3.808 KB   | 59            | 3.808 KB     |
| path-tree        | 6.5198 µs | 59          | 8.704 KB   | 59            | 8.704 KB     |
| matchit          | 7.8498 µs | 140         | 19.07 KB   | 140           | 19.07 KB     |
| xitca-router     | 9.4876 µs | 209         | 26.77 KB   | 209           | 26.77 KB     |
| ntex-router      | 32.175 µs | 201         | 20.8 KB    | 201           | 20.8 KB      |
| route-recognizer | 62.862 µs | 2872        | 192.9 KB   | 2872          | 206.1 KB     |
| routefinder      | 80.606 µs | 525         | 49.66 KB   | 525           | 49.66 KB     |
| actix-router     | 175.21 µs | 2201        | 130.1 KB   | 2201          | 130.1 KB     |

## `wayfind` benches

TODO
