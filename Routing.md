# Routing

Order:

- authority (radix trie, punycode-decode, constraints)
- path (required, radix trie, percent-decode, constraints)
- query (hashmap of vec of params, may or not be present (optional support), static, dynamic)
- method (hashmap per method + hashset for catch-all ?)
- headers (hashmap of vec of headers may or not be present (optional support), static, dynamic)

Questions:
- for path and header, should the key be forced static? or would dynamic keys work? what about patterns, lists, ...

Longer Term (maybe never):

- version (HTTP/1.1, 2, 3, ...)
- scheme (http/https/wss/grpc)
- body (present vs absent) (maybe not a good idea)
- extensions (complex) (probably not needed, but a good escape hatch - would make conflict logic impossible).

## Lookups

Each sub-router will have it's own ID associated with it. (just an incrementing atomic).

Then we'll have a top-level data hashmap (which will use a no-hash method) to chain each sub-id together.

If a given sub-router isn't used, we'll use '*' as a replacement.

So IDs chain will lookup like: `*-353-*-28-*`

Before we go ahead and test this - let's think about each action, and how they work together.

Remember - inserts and deletes can be slow/careful, so long as the method search is fast.

For the time being, let's expose the internal IDs.
WIll make testing much easier.

We should habe a way to identify if there is 'more to come' from a route before returning errors.
So no '?' usage.

e.g.
1. insert "/path" -> OK
2. insert "/path" GET -> path conflicts, but OK because GET makes it unique.
3. insert "/path" -> path conflicts, ERR since nothing else can make this work.

For method routing, we have a tri-state to handle:
- no method filter
- any method
- one or more methods

I'd like in the future for the 'no X filter' states to be handled via type-state.
But then we lose out at our display layer?
Maybe we should collapse the any and none into one state.

### Inserts

So we'd first do the typical radix trie insert.
We traverse the trie, and if there's a conflict, we return that ID instead of our new one.
Expanded routes are handled internally by the prefix trie, so `/({name})` would actually result in 2 seperate nodes.
Need to be careful to handle conflicts across expanded routes.

Then method insert will take the prior path ID, then use it in a hashmap.
If no method is provided, we store a "*" instead of the given method.
If multiple methods are provided, and insert many.
(for deletes, similar logic to the expanded).

FIXME: If the path insert successed, but method insert failed, then what?

Then if the above 2 succeed, we create a data ID chain, and try and insert it.
NOTE: Don't think a conflict can occur at this point?

Then we're done.

### Deletes

So for path, we handle the typical approach.
For safety, we do a search up-front, to grab all expanded routes, and ensure the IDs match.
If they don't, error.
Then we do the actual delete and trie compression, returning the deleted path ID.

Then we do the same for method.
We look up all routes that match, ensure the inserted approach was the same
(e.g. if we insert /hello GET, /hello PUT, we can't delete both via /hello [GET, PUT], needs to be same as input)
Then we remove from the given hashmaps, returning the deleted method ID.

Then we simply create the data ID chain, and delete from the data map.

And we're done.

TODO: Consider ONLY performing searches first for ALL routes, then deleting after we verify everything is OK.

### Searches

This should be the easiest (and cheapest!) action.
We take the user provided path, and do a radix trie lookup.

Then we take the method, and either do a hashmap lookup, or replace it with "*".

Then we make a ID chain, and do a lookup for data.
