# `embedded-can`

![crates.io](https://img.shields.io/crates/v/embedded-can)
![docs.rs](https://docs.rs/embedded-can/badge.svg)

An embedded Controller Area Network (CAN) abstraction layer.
This crate defines generic traits to be implemented by CAN driver and MCU HAL crates.
Eventually to be integrated into `embedded-hal`:
https://github.com/rust-embedded/embedded-hal/pull/212

# Implementations

* [stm32f1xx-hal](https://github.com/timokroeger/stm32f1xx-hal/blob/can-embedded-hal/src/can.rs)
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
