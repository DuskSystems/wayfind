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

These result tables combine both [`criterion`](https://github.com/bheisler/criterion.rs) and [`divan`](https://github.com/nvzqz/divan) output:
- we take the 'point estimate' time from `criterion`.
- we take the 'slowest' alloc/dealloc values from `divan`.

### `matchit` inspired benches

In a router of 130 templates, benchmark matching 130 paths.

#### Default

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| matchit          | 5.4637 µs | 1           | 128 B      | 1             | 128 B        |
| wayfind          | 7.6363 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 8.9698 µs | 0           | n/a        | 0             | n/a          |
| xitca-router     | 14.138 µs | 103         | 13.18 KB   | 103           | 13.18 KB     |
| ntex-router      | 43.049 µs | 306         | 22.97 KB   | 306           | 22.97 KB     |
| route-recognizer | 70.832 µs | 3596        | 195.6 KB   | 3596          | 195.6 KB     |
| actix-router     | 546.53 µs | 6934        | 447.4 KB   | 6934          | 447.4 KB     |

#### String Parameters

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 8.9127 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 10.003 µs | 1           | 128 B      | 1             | 128 B        |
| path-tree        | 10.059 µs | 0           | n/a        | 0             | n/a          |
| xitca-router     | 14.882 µs | 103         | 13.18 KB   | 103           | 13.18 KB     |
| ntex-router      | 43.631 µs | 306         | 22.97 KB   | 306           | 22.97 KB     |
| route-recognizer | 72.031 µs | 3596        | 195.6 KB   | 3596          | 195.6 KB     |
| actix-router     | 544.45 µs | 6934        | 447.4 KB   | 6934          | 447.4 KB     |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 3.9724 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 4.9039 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 6.0203 µs | 81          | 10.36 KB   | 81            | 10.36 KB     |
| xitca-router     | 9.2584 µs | 150         | 18.06 KB   | 150           | 18.06 KB     |
| ntex-router      | 26.716 µs | 142         | 12.09 KB   | 142           | 12.09 KB     |
| route-recognizer | 65.154 µs | 2813        | 184.2 KB   | 2813          | 197.4 KB     |
| actix-router     | 185.50 µs | 2142        | 121.4 KB   | 2142          | 121.4 KB     |

#### String Parameters

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 4.5999 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 5.4243 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 8.1373 µs | 81          | 10.36 KB   | 81            | 10.36 KB     |
| xitca-router     | 9.5970 µs | 150         | 18.06 KB   | 150           | 18.06 KB     |
| ntex-router      | 27.036 µs | 142         | 12.09 KB   | 142           | 12.09 KB     |
| route-recognizer | 65.873 µs | 2813        | 184.2 KB   | 2813          | 197.4 KB     |
| actix-router     | 185.70 µs | 2142        | 121.4 KB   | 2142          | 121.4 KB     |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
For example, having a top-level constrained wildcard route like `/{*path:constraint}`.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
