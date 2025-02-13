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
| matchit          | 5.0940 µs | 1           | 128 B      | 1             | 128 B        |
| wayfind          | 7.1608 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 8.7696 µs | 0           | n/a        | 0             | n/a          |
| xitca-router     | 13.282 µs | 103         | 13.18 KB   | 103           | 13.18 KB     |
| ntex-router      | 40.944 µs | 1706        | 173.8 KB   | 406           | 23.87 KB     |
| route-recognizer | 67.603 µs | 3596        | 195.6 KB   | 3596          | 195.6 KB     |
| actix-router     | 506.54 µs | 9818        | 1.488 MB   | 7140          | 449.2 KB     |

#### String Parameters

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 8.1992 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 9.3138 µs | 1           | 128 B      | 1             | 128 B        |
| path-tree        | 9.7952 µs | 0           | n/a        | 0             | n/a          |
| xitca-router     | 13.885 µs | 103         | 13.18 KB   | 103           | 13.18 KB     |
| ntex-router      | 41.460 µs | 1706        | 173.8 KB   | 406           | 23.87 KB     |
| route-recognizer | 68.661 µs | 3596        | 195.6 KB   | 3596          | 195.6 KB     |
| actix-router     | 509.55 µs | 9818        | 1.488 MB   | 7140          | 449.2 KB     |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 3.6237 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 4.7438 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 5.6865 µs | 81          | 10.36 KB   | 81            | 10.36 KB     |
| xitca-router     | 8.6405 µs | 150         | 18.06 KB   | 150           | 18.06 KB     |
| ntex-router      | 25.330 µs | 1290        | 135.8 KB   | 224           | 12.83 KB     |
| route-recognizer | 61.766 µs | 2813        | 184.2 KB   | 2813          | 197.4 KB     |
| actix-router     | 171.85 µs | 3766        | 595.8 KB   | 2258          | 122.4 KB     |

#### String Parameters

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 4.2385 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 5.2886 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 7.6726 µs | 81          | 10.36 KB   | 81            | 10.36 KB     |
| xitca-router     | 8.9593 µs | 150         | 18.06 KB   | 150           | 18.06 KB     |
| ntex-router      | 25.629 µs | 1290        | 135.8 KB   | 224           | 12.83 KB     |
| route-recognizer | 62.677 µs | 2813        | 184.2 KB   | 2813          | 197.4 KB     |
| actix-router     | 172.93 µs | 3766        | 595.8 KB   | 2258          | 122.4 KB     |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
For example, having a top-level constrained wildcard route like `/{*path:constraint}`.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
