#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::{bind_interrupts, peripherals, time::mhz, usb, Config};
use embassy_usb::class::cdc_acm;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USB =>usb::InterruptHandler<peripherals::USB>;
});

#[embassy_executor::task]
async fn usb() {
    loop {
        
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut mcu_config: Config = Default::default();
    let rcc = &mut mcu_config.rcc;
    rcc.hse = Some(mhz(8));
    rcc.bypass_hse = false;
    rcc.usb_pll = true;
    rcc.hsi48 = false;
    rcc.sys_ck = Some(mhz(48));
    rcc.hclk = Some(mhz(48));
    rcc.pclk = Some(mhz(48));
    let p = embassy_stm32::init(mcu_config);

    let usb_driver = embassy_stm32::usb::Driver::new(p.USB, Irqs, p.PA12, p.PA11);
    let usb_config = embassy_usb::Config::new(0xc0de, 0xcafe);

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 7];

    let mut usb_state = cdc_acm::State::new();
    let mut usb_builder = embassy_usb::Builder::new(
        usb_driver,
        usb_config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );

    let mut class = cdc_acm::CdcAcmClass::new(&mut usb_builder, &mut usb_state, 64);
    let mut usb = usb_builder.build();

    let usb_fut = usb.run();

    let echo_fut = async {
        loop {
            class.wait_connection().await;
            info!("Connected");
            // let _ = echo(&mut class).await;
            // info!("Disconnected");
        }
    };
    join(usb_fut, echo_fut).await;
}
