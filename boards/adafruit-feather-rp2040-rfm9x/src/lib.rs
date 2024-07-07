#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use hal::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_GD25Q64CS;

pub use hal::pac;

hal::bsp_pins!(
    Gpio0 {
        name: tx,
        aliases: { FunctionUart, PullNone: UartTx }
    },
    Gpio1 {
        name: rx,
        aliases: { FunctionUart, PullNone: UartRx }
    },
    Gpio2 {
        name: sda,
        aliases: { FunctionI2C, PullUp: Sda }
    },
    Gpio3 {
        name: scl,
        aliases: { FunctionI2C, PullUp: Scl }
    },
    Gpio4 { name: neopixel },
    Gpio5 { name: d5 },
    Gpio6 { name: d6 },
    Gpio7 { name: button },
    Gpio8 {
        name: miso,
        aliases: { FunctionSpi, PullNone: Miso }
    },
    Gpio9 { name: d9 },
    Gpio10 { name: d10 },
    Gpio11 { name: d11 },
    Gpio12 { name: d12 },
    Gpio13 { name: d13 },
    Gpio14 {
        name: sclk,
        aliases: { FunctionSpi, PullNone: Sclk }
    },
    Gpio15 {
        name: mosi,
        aliases: { FunctionSpi, PullNone: Mosi }
    },
    Gpio16 { name: rfm_cs },
    Gpio17 { name: rfm_rst },
    Gpio18 { name: rfm_io5 },
    Gpio19 { name: rfm_io3 },
    Gpio20 { name: rfm_io4 },
    Gpio21 { name: rfm_io0 },
    Gpio22 { name: rfm_io1 },
    Gpio23 { name: rfm_io2 },
    Gpio24 { name: d24 },
    Gpio25 { name: d25 },
    Gpio26 { name: a0 },
    Gpio27 { name: a1 },
    Gpio28 { name: a2 },
    Gpio29 { name: a3 },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
