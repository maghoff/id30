Id30 is an encoding scheme for 30 bit identifiers that look like the
following: `bpv3uq`, `zvaec2`, `rfmbyz`, `jwygvk`, `000000`, `zzzzzz`. It is
designed for use as opaque identifiers in URLs, that can be read and written
comfortably, and is a good choice for user-facing IDs when you don't need a
key space of more than 30 bits, giving you just over 10⁹ different IDs. For
comparison, YouTube surpassed this only in 2019, and is estimated to be
hosting about 4*10⁹ videos in 2024. User-facing IDs should be chosen such
that they do not reveal an underlying sequence, and, indeed, Id30 looks best
for randomly generated IDs.

This repository is an implementation of Id30 in Rust as a library crate and a
utility binary for easy conversion between Id30 strings and the corresponding
integers.

To use the library, add it as a dependency to your Rust project (`cargo add
id30`) and read [the id30 documentation](https://docs.rs/id30/latest/id30/).

To use the utility binary, build it from the source repository or install it
from crates.io via cargo (`cargo install id30 --features=rand_std`). Then, run
`id30 --help` for more details.
