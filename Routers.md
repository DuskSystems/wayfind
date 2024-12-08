# Routers

We take a modular approach to routing, rather than a typical hierarchial approach.

Each sub router will have an ID.
Sub Router IDs will be joined together to create chain, that will be used in a hashmap.
For example, a chain may look like `*-123-99-*-*`.
The hashmap is where the data lives, i.e. `T`.

The interface for each router will look like:
- insert -> RouteId
- find -> RouteId or Vec<RouteId>
- delete -> ()
- search -> RouteData

Search is the most critical function, as a typical router will be search MUCH more often than it is edited.
We optimize for search performance where possible.

The only required sub router is `PathRouter`.
All others are optional, and can be skipped.
Skippable sub router IDs will be `Option<usize>`, displayed as `*`.

Chains are built up over time.
1. `AuthorityID`  -> `Option<usize>`
2. `PathId`       -> `Option<usize>, usize`
3. `QueryId`      -> `Option<usize>, usize, Option<usize>`
4. `MethodId`     -> `Option<usize>, usize, Option<usize>, Option<usize>`
5. `HeaderId`     -> `Option<usize>, usize, Option<usize>, Option<usize>, Option<usize>`

With this approach, we can filter as we go.
And shortcuts are simple.
Optionals can easily be converted to `None` when skipping a router.

## Data Structures

## 1. Authority Router

A radix trie, where `.` is the seperator.
Support for dynamics `{name}.domain` and wildcards `{*name}.domain`.
Support for constraints `{name:ascii}.domain`.

## 2. Path Router

A radix trie, where `/` is the seperator.
Support for dynamics `/{name}` and wildcards `/{*name}`.
Support for optional groups `/({name})`.
Support for constraints `/{name:ascii}`.

## 3. Query Router

TODO.

## 4. Method Router

TODO: Bitset or HashMap approach?

## 5. Header Router

TODO.
