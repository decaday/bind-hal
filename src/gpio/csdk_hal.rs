//! stm32 cSDK HAL liked  implication

#![macro_use]
// use core::convert::Infallible;
use crate::gpio::{Pull, Speed, Level};

use py32csdk_hal_sys as hal;


impl Pull {
    const fn to_c_macro(self) -> u32 {
        match self {
            Pull::None => hal::GPIO_NOPULL,
            Pull::Up => hal::GPIO_PULLUP,
            Pull::Down => hal::GPIO_PULLDOWN,
        }
    }
}

impl Speed {
    const fn to_c_macro(self) -> u32 {
        match self {
            Speed::Low => hal::GPIO_SPEED_FREQ_LOW,
            Speed::Medium => hal::GPIO_SPEED_FREQ_MEDIUM,
            Speed::High => hal::GPIO_SPEED_FREQ_HIGH,
            Speed::VeryHigh => hal::GPIO_SPEED_FREQ_VERY_HIGH,
        }
    }
}

/// Any pin.
/// for example,{hal::GPIOB, hal::GPIO_PIN_4, xxx}
pub struct AnyPin {
    port: *mut hal::GPIO_TypeDef,
    pin: u16,
    c_init_type: hal::GPIO_InitTypeDef,
}


impl AnyPin{
    /// Form csdk macros like GPIOB GPIO_PIN_4
    pub fn new_from_c_macros(port: *mut hal::GPIO_TypeDef, pin: u16) -> Self {
        let c_init_type = hal::GPIO_InitTypeDef {
            Pin: pin as u32,
            Mode: hal::GPIO_MODE_OUTPUT_PP,
            Pull: hal::GPIO_NOPULL,
            Speed: hal::GPIO_SPEED_FREQ_LOW,
            Alternate: 0,
        };

        Self::open_clk_from_c_macro(port);
        Self{ port, pin, c_init_type }
    }

    /// e.g. ‘B’, '4', no 'GPIO_PIN_4'!
    pub fn new(port_char: char, pin_num: u8) -> Self {
        assert!(pin_num < 16, "Pin num out of range(0-15)!");

        // calculate the GPIO_PIN_x
        let pin = 2i32.pow(pin_num as u32) as u16;

        let port = match port_char{
            #[cfg(feature = "peri-gpioa")]
            'A' | 'a' => hal::GPIOA,
            #[cfg(feature = "peri-gpiob")]
            'B' | 'b' => hal::GPIOB,
            #[cfg(feature = "peri-gpiof")]
            'F' | 'f' => hal::GPIOF,
            _ => panic!("Unknown port char {port_char}, e.g.'B' "),
        };

        let c_init_type = hal::GPIO_InitTypeDef {
            Pin: pin as u32,
            Mode: hal::GPIO_MODE_OUTPUT_PP,
            Pull: hal::GPIO_NOPULL,
            Speed: hal::GPIO_SPEED_FREQ_LOW,
            Alternate: 0,
        };

        Self::open_clk_from_c_macro(port);
        Self{ port, pin, c_init_type }
    }

    fn open_clk_from_c_macro(port: *mut hal::GPIO_TypeDef){
        unsafe {
            match port{
                #[cfg(feature = "peri-gpioa")]
                hal::GPIOA => (),
                #[cfg(feature = "peri-gpiob")]
                hal::GPIOB => hal::HAL_RCC_GPIOB_CLK_ENABLE(),
                #[cfg(feature = "peri-gpiof")]
                hal::GPIOF => (),
                _ => (),
            };
        }
    }

    pub fn open_clk(&mut self){
        Self::open_clk_from_c_macro(self.port)
    }

    /// Put the pin into input mode.
    ///
    /// The internal weak pull-up and pull-down resistors will be enabled according to `pull`.
    #[inline(never)]
    pub fn set_as_input(&mut self, pull: Pull, speed: Speed) {
        self.c_init_type.Speed = speed.to_c_macro();
        self.c_init_type.Mode = hal::GPIO_MODE_INPUT;
        self.c_init_type.Pull = pull.to_c_macro();
        unsafe {
            hal::HAL_GPIO_Init(self.port,
                               &mut self.c_init_type as *mut hal::GPIO_InitTypeDef);
        }
    }

    /// Put the pin into push-pull output mode.
    ///
    /// The pin level will be whatever was set before (or low by default). If you want it to begin
    /// at a specific level, call `set_high`/`set_low` on the pin first.
    ///
    /// The internal weak pull-up and pull-down resistors will be disabled.
    #[inline(never)]
    pub fn set_as_output(&mut self, speed: Speed) {
        self.c_init_type.Speed = speed.to_c_macro();
        self.c_init_type.Mode = hal::GPIO_MODE_OUTPUT_PP;
        self.c_init_type.Pull = hal::GPIO_NOPULL;
        unsafe {
            hal::HAL_GPIO_Init(self.port,
                               &mut self.c_init_type as *mut hal::GPIO_InitTypeDef);
        }
    }

    /// Put the pin into analog mode
    ///
    /// This mode is used by ADC and COMP but usually there is no need to set this manually
    /// as the mode change is handled by the driver.
    #[inline]
    pub fn set_as_analog(&mut self) {
        self.c_init_type.Speed = hal::GPIO_SPEED_FREQ_LOW;
        self.c_init_type.Mode = hal::GPIO_MODE_ANALOG;
        self.c_init_type.Pull = hal::GPIO_NOPULL;
        unsafe {
            hal::HAL_GPIO_Init(self.port,
                               &mut self.c_init_type as *mut hal::GPIO_InitTypeDef);
        }
    }

    /// Put the pin into AF mode, unchecked.
    ///
    /// This puts the pin into the AF mode, with the requested number and AF type. This is
    /// completely unchecked, it can attach the pin to literally any peripheral, so use with care.
    #[inline]
    pub fn set_as_af_unchecked(&mut self, af_num: u8) {
        self.c_init_type.Mode = hal::GPIO_MODE_AF_PP;
        self.c_init_type.Speed = hal::GPIO_SPEED_FREQ_LOW;
        self.c_init_type.Pull = hal::GPIO_NOPULL;
        self.c_init_type.Alternate = af_num as u32;
        unsafe {
            hal::HAL_GPIO_Init(self.port, &mut self.c_init_type);
        }
    }

    /// Get whether the pin input level is high.
    #[inline]
    pub fn is_high(&self) -> bool {
        unsafe { hal::HAL_GPIO_ReadPin(self.port, self.pin) == hal::GPIO_PinState_GPIO_PIN_SET }
    }

    /// Get whether the pin input level is low.
    #[inline]
    pub fn is_low(&self) -> bool {
        unsafe { hal::HAL_GPIO_ReadPin(self.port, self.pin) == hal::GPIO_PinState_GPIO_PIN_RESET }
    }

    /// Get the current pin input level.
    #[inline]
    pub fn get_level(&self) -> Level {
        if self.is_high() {
            Level::High
        } else {
            Level::Low
        }
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        unsafe {
            hal::HAL_GPIO_WritePin(self.port, self.pin, hal::GPIO_PinState_GPIO_PIN_SET);
        }
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        unsafe {
            hal::HAL_GPIO_WritePin(self.port, self.pin, hal::GPIO_PinState_GPIO_PIN_RESET);
        }
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        };
    }

    /// Toggle the output level.
    #[inline]
    pub fn toggle(&mut self) {
        unsafe {
            hal::HAL_GPIO_TogglePin(self.port, self.pin);
        }
    }
}