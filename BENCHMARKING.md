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

As such, we provide 2 sets of results per benchmark:
- one with the default behaviour of the router.
- one with the parameters extracted to `SmallVec<[(&str, &str); 4]>`.

We use [`divan`](https://github.com/nvzqz/divan) for our benchmarks, taking the 'median' results from its output to create the following tables.

### `matchit` inspired benches

In a router of 130 templates, benchmark matching 130 paths.

#### Default

| Library          | Time     | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|---------:|------------:|-----------:|--------------:|-------------:|
| matchit          | 5.415 µs | 1           | 128 B      | 1             | 128 B        |
| wayfind          | 6.915 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 8.915 µs | 0           | n/a        | 0             | n/a          |
| xitca-router     | 14.03 µs | 103         | 13.18 KB   | 103           | 13.18 KB     |
| ntex-router      | 47.38 µs | 306         | 22.97 KB   | 306           | 22.97 KB     |
| route-recognizer | 73.97 µs | 3596        | 195.6 KB   | 3596          | 195.6 KB     |
| actix-router     | 595.7 µs | 6934        | 447.4 KB   | 6934          | 447.4 KB     |

#### String Parameters

| Library          | Time     | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|---------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 7.582 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 9.436 µs | 1           | 128 B      | 1             | 128 B        |
| path-tree        | 9.915 µs | 0           | n/a        | 0             | n/a          |
| xitca-router     | 14.86 µs | 103         | 13.18 KB   | 103           | 13.18 KB     |
| ntex-router      | 47.74 µs | 306         | 22.97 KB   | 306           | 22.97 KB     |
| route-recognizer | 75.22 µs | 3596        | 195.6 KB   | 3596          | 195.6 KB     |
| actix-router     | 560.7 µs | 6934        | 447.4 KB   | 6934          | 447.4 KB     |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time     | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|---------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 3.832 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 4.957 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 6.076 µs | 81          | 10.36 KB   | 81            | 10.36 KB     |
| xitca-router     | 9.321 µs | 150         | 18.06 KB   | 150           | 18.06 KB     |
| ntex-router      | 28.11 µs | 142         | 12.09 KB   | 142           | 12.09 KB     |
| route-recognizer | 67.63 µs | 2813        | 184.2 KB   | 2813          | 197.4 KB     |
| actix-router     | 253.3 µs | 2142        | 121.4 KB   | 2142          | 121.4 KB     |

#### String Parameters

| Library          | Time     | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|---------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 4.249 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 5.457 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 7.867 µs | 81          | 10.36 KB   | 81            | 10.36 KB     |
| xitca-router     | 9.695 µs | 150         | 18.06 KB   | 150           | 18.06 KB     |
| ntex-router      | 28.40 µs | 142         | 12.09 KB   | 142           | 12.09 KB     |
| route-recognizer | 68.49 µs | 2813        | 184.2 KB   | 2813          | 197.4 KB     |
| actix-router     | 193.3 µs | 2142        | 121.4 KB   | 2142          | 121.4 KB     |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
For example, having a top-level constrained wildcard route like `/{*path:constraint}`.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
