#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_abort;
extern crate max31865;
extern crate stm32f103xx_hal as dev_hal;
#[macro_use]
extern crate light_cli;

use core::fmt::Write;
use light_cli::{LightCliInput, LightCliOutput};
use dev_hal::spi::Spi;
use dev_hal::serial::Serial;
use dev_hal::timer::Timer;
use dev_hal::prelude::*;
use light_cli::heapless::consts::*;
use rt::ExceptionFrame;
use max31865::{Max31865, SensorType, FilterMode};

entry!(main);

fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = dev_hal::stm32f103xx::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut heater = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let nss = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
    let miso = gpiob.pb14;
    let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);
    let rdy = gpioa.pa8;

    let spi1 = Spi::spi2(
        dp.SPI2,
        (sck, miso, mosi),
        max31865::MODE,
        100_000.hz(),
        clocks,
        &mut rcc.apb1,
    );

    let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let rx = gpiob.pb7;

    let serial = Serial::usart1(
        dp.USART1,
        (tx,rx),
        &mut afio.mapr,
        115_200.bps(),
        clocks,
        &mut rcc.apb2
    );

    let (mut tx, mut rx) = serial.split();

    let mut cl_in : LightCliInput<U32> = LightCliInput::new();
    let mut cl_out = LightCliOutput::new(&mut tx);

    writeln!(cl_out, "Starting step response");

    let mut max31865 = Max31865::new(spi1, nss, rdy).unwrap();
    max31865.set_calibration(43234).unwrap();
    max31865.configure(true, true, false, SensorType::ThreeWire, FilterMode::Filter50Hz).unwrap();

    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    let mut run = false;
    let mut last = 0;

    loop {
        let _ = cl_out.flush();
        let _ = cl_in.fill(&mut rx);

        if max31865.is_ready().unwrap() {
            last = max31865.read_default_conversion().unwrap();
        }

        if timer.wait() == Ok(()) {
            // update heater state
            if run { heater.set_high() } else { heater.set_low() }

            // output current state
            writeln!(cl_out, "Status Run={} Last={}", run, last);
        }

        lightcli!(cl_in, cl_out, cmd, key, val, [
            "START" => [] => { run = true; };
            "STOP" => [] => { run = false; }
        ]);
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef)
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}