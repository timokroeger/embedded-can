# `embedded-can`

An embedded Controller Area Network (CAN) abstraction layer.
This crate defines generic traits to be implemented by CAN driver and MCU HAL crates.
Eventually to be integrated into `embedded-hal`:
https://github.com/rust-embedded/embedded-hal/pull/212

# Implementations

* [stm32f1xx-hal](https://github.com/timokroeger/stm32f1xx-hal/blob/38f168a7593cf34d426d20b310575df82134e9b9/src/can.rs#L1078-L1124)
* [pcan-basic](https://github.com/timokroeger/pcan-basic-rs/blob/e675be646c501fc63dfdde893e267d14b9c0e0af/pcan-basic/src/lib.rs#L194-L233)

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
