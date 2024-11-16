# Routers

We take a modular approach to routing, rather than a typical hierarchial approach.

Each sub router will have an ID.
Sub Router IDs will be joined together to create chain, that will be used in a map.
For example, a chain may look like `*-123-99`.
The map is where the data lives, i.e. `T`.

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
3. `MethodId`     -> `Option<usize>, usize, Option<usize>`

With this approach, we can filter as we go.
And shortcuts are simple.
Optionals can easily be converted to `None` when skipping a router.

## At a glance.

### 1. Authority Router

A tree, where `.` is the seperator.
Punycode decoding.
Support for dynamics `{name}.com` and wildcards `{*name}.com`.
Support for constraints `{name:my_constraint}.com`.

Most specific match wins.
If a constraint fails, we keep trying to match.

### 2. Path Router

A tree, where `/` is the seperator.
Percent decoding.
Support for dynamics `/{name}` and wildcards `/{*name}`.
Support for optional groups `/({name})`.
Support for constraints `/{name:my_constraint}`.

Most specific match wins.

### 3. Method Router

A bitset over a `u16`.
Custom methods not allowed.

Most specific match wins.

If a method lookup fails, we need to return a 405 error.
This includes a list of what methods actually ARE allowed. (maybe pre-compute this?)

## Inserts

```rust
struct Route {
  authority: Option<String>,
  path: String,
  methods: Option<Vec<String>>
}
```

### 1. Router

1. Take a `Route` as input.
2. If `authority`, call `AuthorityRouter::insert(...)`

### 2. Authority Router

1. Punycode decode input. (error if already decoded/partial)
2. Parse into parts.
3. Generate Authority ID.
4. Walk radix trie, insert or create nodes as go.
5. Reach final part, insert data with ID or return existing data if already exists.

### 3. Router

1. If we called `AuthorityRouter`, expect a Result<Autho>

### 4. Path Router

1. Percent decode input. (error if already decoded/partial)
2. Expand optional routes.
3. Parse into parts
4. Generate Path ID.
5. Walk radix trie per expanded route, insert or create nodes as go.
6. Reach final part, insert data (composite with authority ID) or return existing data if already exists.

### 5. Router

TODO

### 6. Method Router

TODO

### 7. Router

TODO

## Searches

...

## Deletes

...
