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
| matchit          | 5.0418 µs | 1           | 128 B      | 1             | 128 B        |
| wayfind          | 7.1736 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 8.7682 µs | 0           | n/a        | 0             | n/a          |
| xitca-router     | 13.233 µs | 103         | 13.18 KB   | 103           | 13.18 KB     |
| ntex-router      | 41.746 µs | 1706        | 173.8 KB   | 406           | 23.87 KB     |
| route-recognizer | 67.308 µs | 3596        | 195.6 KB   | 3596          | 195.6 KB     |
| actix-router     | 501.32 µs | 9818        | 1.488 MB   | 7140          | 449.2 KB     |

#### String Parameters

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 8.2066 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 9.3431 µs | 1           | 128 B      | 1             | 128 B        |
| path-tree        | 9.7824 µs | 0           | n/a        | 0             | n/a          |
| xitca-router     | 14.003 µs | 103         | 13.18 KB   | 103           | 13.18 KB     |
| ntex-router      | 42.307 µs | 1706        | 173.8 KB   | 406           | 23.87 KB     |
| route-recognizer | 68.009 µs | 3596        | 195.6 KB   | 3596          | 195.6 KB     |
| actix-router     | 508.03 µs | 9818        | 1.488 MB   | 7140          | 449.2 KB     |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 3.6072 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 4.6104 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 5.5912 µs | 81          | 10.36 KB   | 81            | 10.36 KB     |
| xitca-router     | 8.6416 µs | 150         | 18.06 KB   | 150           | 18.06 KB     |
| ntex-router      | 25.119 µs | 1290        | 135.8 KB   | 224           | 12.83 KB     |
| route-recognizer | 62.131 µs | 2813        | 184.2 KB   | 2813          | 197.4 KB     |
| actix-router     | 171.74 µs | 3766        | 595.8 KB   | 2258          | 122.4 KB     |

#### String Parameters

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 4.0955 µs | 0           | n/a        | 0             | n/a          |
| path-tree        | 5.1748 µs | 0           | n/a        | 0             | n/a          |
| matchit          | 7.5645 µs | 81          | 10.36 KB   | 81            | 10.36 KB     |
| xitca-router     | 8.9677 µs | 150         | 18.06 KB   | 150           | 18.06 KB     |
| ntex-router      | 25.515 µs | 1290        | 135.8 KB   | 224           | 12.83 KB     |
| route-recognizer | 63.057 µs | 2813        | 184.2 KB   | 2813          | 197.4 KB     |
| actix-router     | 172.61 µs | 3766        | 595.8 KB   | 2258          | 122.4 KB     |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
For example, having a top-level constrained wildcard route like `/{*path:constraint}`.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
