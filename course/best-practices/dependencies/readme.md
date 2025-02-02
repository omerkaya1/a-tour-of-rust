# dependencies

specify the exact version:

axum = "=0.6.19"

cargo also is able to make use of vendoring - it's important for long-term support

## managing your own dependencies

it's possible to specify the version, the git repo address etc.

## overriding dependencies

example:

```toml
[patch.crates-io]
uuid = { path = "../path/to/uuid" }
```