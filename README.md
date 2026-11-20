# twinwrap-demo

Two small libraries wrapped for PyPI and npm by
[twinwrap](https://github.com/godofecht/twinwrap), used to run its
generated GitHub workflows on every platform.

- `rust-stats/`: a Rust crate
- `flow-stats/`: a Flow 2.x library

Each has its own workflow under `.github/workflows/`. A manual run builds
both packages, runs the Python and Node conformance tests, and builds and
checks a wheel per platform. Nothing here is published: the publishing
jobs only run on release tags, and this repo has no publishers set up.
