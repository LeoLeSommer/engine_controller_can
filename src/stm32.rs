use crate::can_input::{CanInput, CanMessage};
use crate::gear_output::GearOutput;
use bxcan::filter::Mask32;
use bxcan::{Fifo, Id};
use hal::flash::Parts;
use hal::hal::digital::v2::OutputPin;
use hal::pac;
use hal::prelude::*;
use hal::rcc::Rcc;
use nb::block;
use stm32f3xx_hal as hal;
use stm32f3xx_hal::can::{RxPin, TxPin};

pub struct STM32CanInput<Tx, Rx>
where
    Tx: TxPin,
    Rx: RxPin,
{
    can: bxcan::Can<hal::can::Can<Tx, Rx>>,
}

impl<Tx, Rx> STM32CanInput<Tx, Rx>
where
    Tx: TxPin,
    Rx: RxPin,
{
}

impl<Tx, Rx> STM32CanInput<Tx, Rx>
where
    Tx: TxPin,
    Rx: RxPin,
{
    pub fn new(pac_can: pac::CAN, mut flash: Parts, mut rcc: Rcc, tx: Tx, rx: Rx) -> Self {
        let _clocks = rcc
            .cfgr
            .use_hse(32.MHz())
            .hclk(64.MHz())
            .sysclk(64.MHz())
            .pclk1(32.MHz())
            .pclk2(64.MHz())
            .freeze(&mut flash.acr);

        // Initialize the CAN peripheral
        // Use loopback mode: No pins need to be assigned to peripheral.
        // APB1 (PCLK1): 64MHz, Bit rate: 500kBit/s, Sample Point 87.5%
        // Value was calculated with http://www.bittiming.can-wiki.info/
        let mut can = bxcan::Can::builder(hal::can::Can::new(pac_can, tx, rx, &mut rcc.apb1))
            .set_bit_timing(0x001c_0003)
            .set_loopback(false)
            .set_silent(false)
            .leave_disabled();

        let mut filters = can.modify_filters();

        filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());

        // Enable filters.
        drop(filters);

        // Sync to the bus and start normal operation.
        block!(can.enable_non_blocking()).ok();

        STM32CanInput { can: can }
    }
}

impl<Tx, Rx> CanInput for STM32CanInput<Tx, Rx>
where
    Tx: TxPin,
    Rx: RxPin,
{
    fn receive(&mut self) -> Result<CanMessage, ()> {
        let rcv_frame = block!(self.can.receive()).expect("Cannot receive CAN frame");
        let id = match rcv_frame.id() {
            Id::Standard(id) => id.as_raw(),
            _ => {
                return Err(());
            }
        };

        if let Some(data) = rcv_frame.data() {
            return Ok(CanMessage {
                id: id,
                data: u16::from_le_bytes([data[0], data[1]]),
            });
        } else {
            return Err(());
        }
    }
}

pub struct STM32GearOutput<Pin1, Pin2, Pin3, Pin4>
where
    Pin1: OutputPin,
    Pin2: OutputPin,
    Pin3: OutputPin,
    Pin4: OutputPin,
{
    pin1: Pin1,
    pin2: Pin2,
    pin3: Pin3,
    pin4: Pin4,
}

impl<Pin1, Pin2, Pin3, Pin4> STM32GearOutput<Pin1, Pin2, Pin3, Pin4>
where
    Pin1: OutputPin,
    Pin2: OutputPin,
    Pin3: OutputPin,
    Pin4: OutputPin,
{
    pub fn new(pin1: Pin1, pin2: Pin2, pin3: Pin3, pin4: Pin4) -> Self {
        STM32GearOutput {
            pin1: pin1,
            pin2: pin2,
            pin3: pin3,
            pin4: pin4,
        }
    }
}

impl<Pin1, Pin2, Pin3, Pin4> GearOutput for STM32GearOutput<Pin1, Pin2, Pin3, Pin4>
where
    Pin1: OutputPin,
    Pin2: OutputPin,
    Pin3: OutputPin,
    Pin4: OutputPin,
{
    fn set_pin1(&mut self, value: bool) -> Result<(), ()> {
        if value {
            self.pin1.set_high().map_err(|_| ())
        } else {
            self.pin1.set_low().map_err(|_| ())
        }
    }

    fn set_pin2(&mut self, value: bool) -> Result<(), ()> {
        if value {
            self.pin2.set_high().map_err(|_| ())
        } else {
            self.pin2.set_low().map_err(|_| ())
        }
    }

    fn set_pin3(&mut self, value: bool) -> Result<(), ()> {
        if value {
            self.pin3.set_high().map_err(|_| ())
        } else {
            self.pin3.set_low().map_err(|_| ())
        }
    }

    fn set_pin4(&mut self, value: bool) -> Result<(), ()> {
        if value {
            self.pin4.set_high().map_err(|_| ())
        } else {
            self.pin4.set_low().map_err(|_| ())
        }
    }
}
