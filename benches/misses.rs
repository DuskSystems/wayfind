//! The downside to our approach of supporting wildcards and expensive regex capabilities is that it may make
//! misses much more expensive, even if our matches are relatively quick.
//! Ideally, it would be nice to test against naive, hand-rolled implementations of complex routing.
//! e.g. have a static route in matchit, but then run a regex within it's function post match.
//! We may be better off adding these benchmarks to the existing matchit/path-tree benches.
//! TODO
