# Benchmarking

All benchmarks ran on a M1 Pro laptop running Asahi Linux.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind/benchmarks) for a more accurate set of timings.

## Context

For all benchmarks, we percent-decode the path before matching.
After matching, we convert any extracted parameters to strings.

Some routers perform these operations automatically, while others require them to be done manually.

We do this to try and match behaviour as best as possible. This is as close to an "apples-to-apples" comparison as we can get.

## `matchit` inspired benches

In a router of 130 routes, benchmark matching 4 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 475.73 ns | 5           | 329 B      | 5             | 329 B        |
| matchit          | 551.32 ns | 5           | 480 B      | 5             | 512 B        |
| path-tree        | 564.04 ns | 5           | 480 B      | 5             | 512 B        |
| xitca-router     | 646.81 ns | 8           | 864 B      | 8             | 896 B        |
| ntex-router      | 2.2001 µs | 19          | 1.312 KB   | 19            | 1.344 KB     |
| route-recognizer | 3.1331 µs | 161         | 8.569 KB   | 161           | 8.601 KB     |
| routefinder      | 6.1604 µs | 68          | 5.088 KB   | 68            | 5.12 KB      |
| actix-router     | 20.956 µs | 215         | 14 KB      | 215           | 14.03 KB     |

## `path-tree` inspired benches

In a router of 320 routes, benchmark matching 80 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 7.0409 µs | 60          | 3.847 KB   | 60            | 3.847 KB     |
| path-tree        | 8.4434 µs | 60          | 8.727 KB   | 60            | 8.75 KB      |
| matchit          | 9.8761 µs | 141         | 19.09 KB   | 141           | 19.11 KB     |
| xitca-router     | 11.651 µs | 210         | 26.79 KB   | 210           | 26.81 KB     |
| ntex-router      | 35.669 µs | 202         | 20.82 KB   | 202           | 20.84 KB     |
| route-recognizer | 69.671 µs | 2873        | 192.9 KB   | 2873          | 206.1 KB     |
| routefinder      | 87.075 µs | 526         | 49.68 KB   | 526           | 49.71 KB     |
| actix-router     | 185.31 µs | 2202        | 130.1 KB   | 2202          | 130.1 KB     |
