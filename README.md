# `embedded-can`

[![crates.io](https://img.shields.io/crates/v/embedded-can)](https://crates.io/crates/embedded-can)
[![docs.rs](https://docs.rs/embedded-can/badge.svg)](https://docs.rs/embedded-can)

# ⚠️ Deprecation Notice ⚠️

Varations of these traits have been upstreamed to embedded-hal:
* [embedded-hal v0.2.7](https://docs.rs/embedded-hal/0.2.7/embedded_hal/can/index.html)
* [embedded-hal v1.0.0-alpha.6](https://docs.rs/embedded-hal/1.0.0-alpha.6/embedded_hal/can/index.html)

Please implement and use the embedded-hal traits for any new CAN drivers.

# Description

An embedded Controller Area Network (CAN) abstraction layer.
This crate defines generic traits to be implemented by CAN driver and MCU HAL crates.
Eventually to be integrated into `embedded-hal`:
https://github.com/rust-embedded/embedded-hal/pull/212

# Implementations

* [bxcan](https://github.com/stm32-rs/bxcan/pull/6)
* [pcan-basic](https://github.com/timokroeger/pcan-basic-rs/blob/master/pcan-basic/src/lib.rs)

# Usage examples

* [stm32-fwupdate](https://github.com/timokroeger/pcan-basic-rs/blob/master/pcan-basic/examples/stm32-fwupdate.rs)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
