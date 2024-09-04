![oci](https://github.com/DuskSystems/wayfind/actions/workflows/oci.yml/badge.svg)

# `oci` example

This is a toy implementation of an [Open Container Initiative (OCI) Distribution Specification](https://github.com/opencontainers/distribution-spec/blob/v1.1.0/spec.md) registry.

Request handling is inspired by the `hyper-util` examples.

API structure is inspired by `axum`, but without use of `tower`.

All 'pull' [conformance tests](https://github.com/opencontainers/distribution-spec/tree/v1.1.0/conformance) pass.
