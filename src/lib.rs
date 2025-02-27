#![no_main]
#![no_std]

use cortex_m_semihosting::debug;

use defmt_rtt as _; // global logger

use panic_probe as _;

/// Hardfault handler.
///
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with an error. This seems better than the default, which is to spin in a
/// loop.
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error{
    Error,
    Busy,
    Timeout,
}

pub fn init(){
    crate::csdk_hal::init();

    unsafe {
        csdk::HAL_RCC_SYSCFG_CLK_ENABLE();
        csdk::HAL_RCC_PWR_CLK_ENABLE();
    }

    #[cfg(feature = "embassy")]
    crate::time_driver::init();
}

pub mod mode {
    trait SealedMode {}

    /// Operating mode for a peripheral.
    #[allow(private_bounds)]
    pub trait Mode: SealedMode {}

    macro_rules! impl_mode {
        ($name:ident) => {
            impl SealedMode for $name {}
            impl Mode for $name {}
        };
    }

    /// Blocking mode.
    pub struct Blocking;
    /// Async mode.
    pub struct Async;

    impl_mode!(Blocking);
    impl_mode!(Async);
}


pub use py32csdk_hal_sys as csdk;

pub mod gpio;

pub mod power;

#[cfg(feature = "peri-i2c")]
pub mod i2c;

pub mod exti;

pub mod rcc;

pub mod adc;

pub mod dma;

pub mod csdk_hal;

mod time_driver;