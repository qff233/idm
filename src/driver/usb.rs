use embassy_stm32::peripherals;
use embassy_stm32::usb::Driver;
use embassy_usb::{class::cdc_acm::State, Builder, Config};

struct USB<'a> {
    device_descriptor: [u8; 256],
    config_descriptor: [u8; 256],
    bos_descriptor: [u8; 256],
    control_buf: [u8; 7],
    state: State<'a>,
}
impl<'a> USB<'a> {
    fn new(driver: Driver<peripherals::USB>, config: Config) -> USB<'a> {
        let state = State::new();
        let usb = USB {
            device_descriptor: [0; 256],
            config_descriptor: [0; 256],
            bos_descriptor: [0; 256],
            control_buf: [0; 7],
            state,
        };

        let mut builder = embassy_usb::Builder::new(
            driver,
            config,
            &mut usb.device_descriptor,
            &mut usb.config_descriptor,
            &mut usb.bos_descriptor,
            &mut [], // no msos descriptors
            &mut usb.control_buf,
        );

        let mut class = cdc_acm::CdcAcmClass::new(&mut builder, &mut usb_state, 64);
        let mut _usb = builder.build();
        usb
    }
}
