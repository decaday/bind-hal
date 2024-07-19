# bind-hal

[![Crates.io](https://img.shields.io/crates/v/bind-hal.svg)](https://crates.io/crates/bind-hal)

This project aims to provide a more complete HAL (Hardware Abstraction Layer).

The project uses the vendor-provided C SDK and operates peripherals through bindings, then wraps these C APIs for easy use in Rust.

Users can also directly use FFI to perform complex operations without manipulating registers.

### Why use bindings?

Taking STM32 as an example, there are many excellent HALs available: [embassy](https://github.com/embassy-rs/embassy)   [stm32-rs](https://github.com/stm32-rs)



This crate’s performance, ROM, and RAM usage are far inferior to these HALs. 

However, most Rust HALs are maintained by the community or enthusiasts and do not receive vendor support. Especially for microcontrollers with fewer users, there are not enough people to maintain the HAL, or in the end, only basic functions can be used.

This crate requires little maintenance and does not require dealing with registers. Even if there are unwrapped functions, others can easily supplement or directly call FFI.



In the near future, this crate will primarily update SDKs for microcontrollers similar to `STM32 HAL CSDK` or `STM32 StdLib CSDK` aiming to reuse code on similar SDKs.

The CSDK and bindings for py32 are maintained here: [py32csdk-hal-sys](https://github.com/decaday/py32csdk-hal-sys), and this package already includes precompiled stastic library file and `bindings.rs` for quick use. However, if you want to recompile and generate bindings, it will be very troublesome. You need to enable the `recompile` feature.
