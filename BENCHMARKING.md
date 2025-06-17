# Benchmarking

All benchmarks ran on a M1 Pro laptop running Asahi Linux.

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

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| matchit          | 5.541 µs | 1           | 128 B      |
| wayfind          | 7.437 µs | 0           | n/a        |
| path-tree        | 8.999 µs | 0           | n/a        |
| xitca-router     | 14.91 µs | 103         | 13.18 KB   |
| ntex-router      | 47.61 µs | 306         | 22.97 KB   |
| route-recognizer | 75.27 µs | 3596        | 195.6 KB   |
| actix-router     | 564.5 µs | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| matchit          | 7.583 µs | 1           | 128 B      |
| wayfind          | 8.041 µs | 0           | n/a        |
| path-tree        | 10.49 µs | 0           | n/a        |
| xitca-router     | 15.33 µs | 103         | 13.18 KB   |
| ntex-router      | 48.61 µs | 306         | 22.97 KB   |
| route-recognizer | 76.40 µs | 3596        | 195.6 KB   |
| actix-router     | 564.5 µs | 6934        | 447.4 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| wayfind          | 3.666 µs | 0           | n/a        |
| path-tree        | 4.916 µs | 0           | n/a        |
| matchit          | 6.083 µs | 81          | 10.36 KB   |
| xitca-router     | 9.499 µs | 150         | 18.06 KB   |
| ntex-router      | 28.33 µs | 142         | 12.09 KB   |
| route-recognizer | 68.12 µs | 2813        | 184.2 KB   |
| actix-router     | 210.9 µs | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| wayfind          | 4.040 µs | 0           | n/a        |
| path-tree        | 5.624 µs | 0           | n/a        |
| matchit          | 6.999 µs | 81          | 10.36 KB   |
| xitca-router     | 9.749 µs | 150         | 18.06 KB   |
| ntex-router      | 28.58 µs | 142         | 12.09 KB   |
| route-recognizer | 68.66 µs | 2813        | 184.2 KB   |
| actix-router     | 187.1 µs | 2142        | 121.4 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
For example, having a top-level constrained wildcard route like `/{*path:constraint}`.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
