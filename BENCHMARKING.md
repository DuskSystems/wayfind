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
| wayfind          | 3.6072 µs | 1           | 1.28 KB    | 1             | 1.28 KB      |
| path-tree        | 4.6104 µs | 1           | 1.28 KB    | 1             | 1.28 KB      |
| matchit          | 5.5912 µs | 3           | 1.536 KB   | 82            | 11.64 KB     |
| xitca-router     | 8.6416 µs | 151         | 19.34 KB   | 151           | 19.34 KB     |
| ntex-router      | 25.119 µs | 1291        | 137.1 KB   | 225           | 14.11 KB     |
| route-recognizer | 62.131 µs | 2814        | 185.5 KB   | 2814          | 198.6 KB     |
| actix-router     | 171.74 µs | 3767        | 597.1 KB   | 2259          | 123.7 KB     |

#### String Parameters

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 4.0955 µs | 1           | 1.28 KB    | 1             | 1.28 KB      |
| path-tree        | 5.1748 µs | 1           | 1.28 KB    | 1             | 1.28 KB      |
| matchit          | 7.5645 µs | 3           | 1.536 KB   | 82            | 11.64 KB     |
| xitca-router     | 8.9677 µs | 151         | 19.34 KB   | 151           | 19.34 KB     |
| ntex-router      | 25.515 µs | 1291        | 137.1 KB   | 225           | 14.11 KB     |
| route-recognizer | 63.057 µs | 2814        | 185.5 KB   | 2814          | 198.6 KB     |
| actix-router     | 172.61 µs | 3767        | 597.1 KB   | 2259          | 123.7 KB     |

## `wayfind` benches

TODO
