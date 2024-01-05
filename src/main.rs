#![no_std]
#![no_main]

use can_input::CanInput;
use can_input::CanMessage;
use cortex_m_rt::entry;
use gear_output::GearOutput;
use gear_output::GearState;
use gear_output::GearsState;
use hal::pac;
use hal::prelude::*;
use panic_semihosting as _;
use stm32::STM32CanInput;
use stm32::STM32GearOutput;
use stm32f3xx_hal as hal;

pub mod can_input;
pub mod gear_output;
pub mod stm32;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // Configure CAN RX and TX pins (AF9)
    let rx = gpiob
        .pb8
        .into_af_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);
    let tx = gpiob
        .pb9
        .into_af_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);
    let out1 = gpiob
        .pb1
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let out2 = gpiob
        .pb2
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let out3 = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let out4 = gpiob
        .pb4
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // Initial state
    let mut state = GearsState::default();

    // Create hardware interfaces
    let mut can = STM32CanInput::new(dp.CAN, dp.FLASH.constrain(), rcc, tx, rx);
    let mut output = STM32GearOutput::new(out1, out2, out3, out4);

    loop {
        receive_can_message(&mut can, &mut state, &mut output);
    }
}

fn receive_can_message(
    can: &mut dyn CanInput,
    state: &mut GearsState,
    output: &mut dyn GearOutput,
) {
    let message = can.receive();
    if let Ok(CanMessage { id, data }) = message {
        let pgn = ((id as u32) >> 8) & 131071;

        if pgn == 65284 {
            if (data as u32) & 2097152 != 0 {
                // starboard
                let gear_state = match data {
                    0x0 => GearState::Neutral,
                    0x1 => GearState::Forward,
                    0x2 => GearState::Reverse,
                    0x3 => GearState::Stop,
                    _ => state.starboard.clone(),
                };

                state.starboard = gear_state;

                // Update output
                update_output(&state, output);
            } else {
                // port
                let gear_state = match data {
                    0x0 => GearState::Neutral,
                    0x1 => GearState::Forward,
                    0x2 => GearState::Reverse,
                    0x3 => GearState::Stop,
                    _ => state.port.clone(),
                };

                state.port = gear_state;

                // Update output
                update_output(&state, output);
            }
        }
    }
}

fn update_output(state: &GearsState, output: &mut dyn GearOutput) {
    match state.starboard {
        GearState::Forward => {
            output.set_pin1(false).unwrap();
            output.set_pin2(true).unwrap();
        }
        GearState::Neutral => {
            output.set_pin1(false).unwrap();
            output.set_pin2(false).unwrap();
        }
        GearState::Reverse => {
            output.set_pin1(true).unwrap();
            output.set_pin2(false).unwrap();
        }
        GearState::Stop => {
            output.set_pin1(false).unwrap();
            output.set_pin2(false).unwrap();
        }
    }

    match state.port {
        GearState::Forward => {
            output.set_pin3(false).unwrap();
            output.set_pin4(true).unwrap();
        }
        GearState::Neutral => {
            output.set_pin3(false).unwrap();
            output.set_pin4(false).unwrap();
        }
        GearState::Reverse => {
            output.set_pin3(true).unwrap();
            output.set_pin4(false).unwrap();
        }
        GearState::Stop => {
            output.set_pin3(false).unwrap();
            output.set_pin4(false).unwrap();
        }
    }
}
