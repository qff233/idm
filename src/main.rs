#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use defmt::info;
use defmt_rtt as _;

use stm32f0xx_hal as hal;

use hal::usb::{Peripheral, UsbBus};
use hal::{pac, prelude::*};
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

mod usb;

fn init_usb() {

}

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    hal::usb::remap_pins(&mut dp.RCC, &mut dp.SYSCFG);
    let mut rcc = dp
        .RCC
        .configure()
        .hsi48()
        .enable_crs(dp.CRS)
        .sysclk(48.mhz())
        .pclk(24.mhz())
        .freeze(&mut dp.FLASH);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
    };
    let usb_bus = UsbBus::new(usb);
    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("idm")
        .product("idm")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();
    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
        let mut buf = [0u8; 64];
        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                info!("recv");
                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
