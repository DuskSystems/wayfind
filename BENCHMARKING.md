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
| wayfind          | 7.8473 µs | 103         | 6.464 KB   | 103           | 6.464 KB     |
| matchit          | 11.580 µs | 104         | 13.31 KB   | 104           | 13.31 KB     |
| path-tree        | 12.705 µs | 103         | 13.18 KB   | 103           | 13.18 KB     |
| xitca-router     | 15.312 µs | 206         | 26.36 KB   | 206           | 26.36 KB     |
| ntex-router      | 51.882 µs | 409         | 36.16 KB   | 409           | 36.16 KB     |
| route-recognizer | 69.927 µs | 3699        | 208.8 KB   | 3699          | 208.8 KB     |
| routefinder      | 171.12 µs | 2115        | 161.7 KB   | 2115          | 161.7 KB     |
| actix-router     | 513.15 µs | 7037        | 460.6 KB   | 7037          | 460.6 KB     |

## `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 3.9770 µs | 59          | 3.808 KB   | 59            | 3.808 KB     |
| path-tree        | 6.3722 µs | 59          | 8.704 KB   | 59            | 8.704 KB     |
| matchit          | 7.8244 µs | 140         | 19.07 KB   | 140           | 19.07 KB     |
| xitca-router     | 9.6346 µs | 209         | 26.77 KB   | 209           | 26.77 KB     |
| ntex-router      | 32.256 µs | 201         | 20.8 KB    | 201           | 20.8 KB      |
| route-recognizer | 62.956 µs | 2872        | 192.9 KB   | 2872          | 206.1 KB     |
| routefinder      | 80.565 µs | 525         | 49.66 KB   | 525           | 49.66 KB     |
| actix-router     | 173.26 µs | 2201        | 130.1 KB   | 2201          | 130.1 KB     |

## `wayfind` benches

TODO
