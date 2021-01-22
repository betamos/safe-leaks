# Safe Leaks

This crate challenges you to leak non-`'static` data past its
lifetime in safe Rust.

As an adversary, your job to write a _leaking function_ `FnOnce(LeakMe<'a>)`
where you prevent `LeakMe`'s destructor from running. You may use
any _safe_ APIs from `std` (feel free to try with third-party crates too)
to achieve this goal, including panic unwinding.

Note that you cannot simply avoid the destructor through process
termination or an inifinite loop. For the leak to be valid,
**the leaking function must return**. In other words, the code that
follows the input lifetime `'a` must be executed without invoking `LeakMe`'s
destructor.

## Purpose

This crate should not be used in any real application. It's purpose is to
demonstrate and enumerate the different ways that destructors are not
invoked in safe Rust today. In a parallel universe, there is a version
of Rust where destructors are statically guaranteed to run, where you can
create amazing things such as
[scoped async tasks](https://docs.rs/async-scoped/0.6.0/async_scoped/)
(that don't block an entire thread).

## Contributing

See [the test module](src/tests.rs) for current techniques, and run them through
`cargo test`.
If you find a novel technique,
[please send a PR](https://github.com/betamos/safe-leaks).

## License

Licensed under either of [Apache License, Version
2.0](//www.apache.org/licenses/LICENSE-2.0) or [MIT
license](//opensource.org/licenses/MIT) at your option.

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this crate by you,
as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.

Author: Didrik Nordström