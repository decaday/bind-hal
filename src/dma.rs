use core::ffi::c_void;

use crate::csdk;
use crate::csdk::interrupts::interrupt;

const DMA_CHANNEL_COUNT: usize = 16;
static mut DMA_CHANNELS: [Option<*mut csdk::DMA_HandleTypeDef>; DMA_CHANNEL_COUNT] = [None; DMA_CHANNEL_COUNT];

pub struct DmaChannel {
    pub handle: csdk::DMA_HandleTypeDef,
}

pub struct Config {
    init: csdk::DMA_InitTypeDef,
}

impl Config {
    pub fn new() -> Self {
        Self {
            init: csdk::DMA_InitTypeDef{
                Direction: csdk::DMA_PERIPH_TO_MEMORY,
                PeriphInc: csdk::DMA_PINC_DISABLE,
                MemInc: csdk::DMA_MINC_DISABLE,
                PeriphDataAlignment: csdk::DMA_PDATAALIGN_HALFWORD,
                MemDataAlignment: csdk::DMA_MDATAALIGN_HALFWORD,
                Mode: csdk::DMA_CIRCULAR,
                Priority: csdk::DMA_PRIORITY_VERY_HIGH,
            }
        }
    }

    pub fn new_peri_to_mem() -> Self {
        let mut conf = Self::new();
        conf.init.Direction = csdk::DMA_PERIPH_TO_MEMORY;
        conf
    }

    pub fn new_mem_to_peri() -> Self {
        let mut conf = Self::new();
        conf.init.Direction = csdk::DMA_MEMORY_TO_PERIPH;
        conf
    }
}

impl DmaChannel {
    /// 00000：ADC 
    /// 00001：SPI1_TX  00010：SPI1_RX 
    /// 00011：SPI2_TX  00100：SPI2_RX 
    /// 00101：USART1_TX  00110：USART1_RX 
    /// 00111：USART2_TX  01000：USART2_RX 
    /// 01001：I2C_TX  01010：I2C_RX 
    /// 01011：TIM1_CH1  01100：TIM1_CH2  01101：TIM1_CH3  01110：TIM1_CH4 
    /// 01111：TIM1_COM  10000：TIM1_UP  10001：TIM1_TRIG 
    /// 10010：TIM3_CH1 10011：TIM3_CH3  10100：TIM3_CH4 
    /// 10101：TIM3_TRG  10110：TIM3_UP 
    /// 10111：Reserved
    /// 11000：TIM16_CH1  11001：TIM16_UP  11010：TIM17_CH1 
    /// 11011：TIM17_UP 
    pub fn new(config: Config, channel: u8, map_value: u8) -> Result<Self, crate::Error> {
        let mut handle = csdk::DMA_HandleTypeDef {
            Instance: csdk::DMA1_Channel1,
            Init: config.init,
            Lock: 0,
            State: 0,
            Parent: core::ptr::null_mut(),
            XferCpltCallback: None,
            XferHalfCpltCallback: None,
            XferErrorCallback: None,
            XferAbortCallback: None,
            ErrorCode: 0,
            DmaBaseAddress: core::ptr::null_mut(),
            ChannelIndex: 0,
        };
        
        unsafe {
            csdk::HAL_RCC_DMA_CLK_ENABLE();

            handle.Instance = match channel {
                1 => {
                    (*csdk::SYSCFG).CFGR3 &= !(0b11111);
                    (*csdk::SYSCFG).CFGR3 |= map_value as u32;
                    csdk::DMA1_Channel1
                },
                2 => {
                    (*csdk::SYSCFG).CFGR3 &= !(0b11111 << 8);
                    (*csdk::SYSCFG).CFGR3 |= (map_value as u32) << 8;
                    csdk::DMA1_Channel2
                },
                3 => {
                    (*csdk::SYSCFG).CFGR3 &= !(0b11111 << 16);
                    (*csdk::SYSCFG).CFGR3 |= (map_value as u32) << 16;
                    csdk::DMA1_Channel3
                },
                _ => panic!(),
            };
            // defmt::println!("cfgr3:{:#b}", (*csdk::SYSCFG).CFGR3);

            
            handle.State = csdk::HAL_DMA_StateTypeDef_HAL_DMA_STATE_READY;
            // check(csdk::HAL_DMA_DeInit(&mut handle))?;
            defmt::println!("dma state 1 {} {} {}", handle.State, 1, handle.ErrorCode);
            // handle.ErrorCode = 100;
            
            let result = csdk::HAL_DMA_Init(&mut handle);
            // check()?;
            defmt::println!("dma state 2 {} {} {}", handle.State, result, handle.ErrorCode);
        }
        
        Ok(Self { handle })
    }

    pub fn link(&mut self, handle: &mut impl HasDmaField){
        handle.set_dma_field(self);
        self.handle.Parent = handle.get_handle_ptr();
    }

}

pub trait HasDmaField {
    fn set_dma_field(&mut self, dma_handle: &mut DmaChannel);

    fn get_handle_ptr(&mut self) -> *mut c_void;
}


#[interrupt]
unsafe fn DMA1_CHANNEL1() {
    on_irq();
}

#[interrupt]
unsafe fn DMA1_CHANNEL2_3() {
    on_irq();
}

unsafe fn on_irq() {
    // WIP
    let isr = (*csdk::DMA1).ISR;

    let channel_id = if (isr & 1 ) != 0 { 
        Some(0) 
    } else if ( isr & (1 << 4) ) != 0 { 
        Some(1)
    } else if ( isr & (1 << 8) ) != 0 { 
        Some(2)
    } else {
        None
    };
    defmt::println!("channel_id: {}", channel_id);
    match channel_id {
        Some(id) => match DMA_CHANNELS[id] {
            Some(ptr) => csdk::HAL_DMA_IRQHandler(ptr),
            None => (),
        },
        None => (),
    }
}