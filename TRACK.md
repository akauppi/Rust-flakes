# Track Rust progress

Collection of items the author is interested to see the status of.

## `gen` in edition "2024"?

- [ ] [Tracking issue for `gen` blocks and functions](https://github.com/rust-lang/rust/issues/117078)

	Can by tried, with:
	
	```
	# cargo.toml
	edition: "2024"
	#![feature(gen_blocks)]
	```
	
	- [ ] Make a sample in the flakes :)


## Getting `stream` into the `std::` module

Do we need it to be?

Do `futures-rs` and `tokio` provide enough?


## `spmc` channel?

- Last (2024) entry of ["add an mpmc channel"](https://github.com/rust-lang/rfcs/issues/848)

   The `Pool` approach could benefit from this.


## Trait aliases

- ["Tracking issue for trait aliases"](https://github.com/rust-lang/rust/issues/41517)

   