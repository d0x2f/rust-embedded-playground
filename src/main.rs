#![no_main]
#![no_std]

mod leds;

use panic_halt as _;

use cortex_m::interrupt as cortex_interrupt;
use cortex_m::peripheral::syst::SystClkSource;

use stm32h7xx_hal::gpio::PinState;
use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::{interrupt, pac, pac::Interrupt, pac::NVIC};

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use leds::Leds;

static LEDS: Mutex<RefCell<Option<Leds>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
  let p = pac::CorePeripherals::take().unwrap();
  let dp = pac::Peripherals::take().unwrap();

  let mut syst = p.SYST;
  let mut nvic = p.NVIC;

  let pwr = dp.PWR.constrain();
  let pwrcfg = pwr.freeze();
  let rcc = dp.RCC.constrain();
  let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

  let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
  let gpioh = dp.GPIOH.split(ccdr.peripheral.GPIOH);
  let gpioi = dp.GPIOI.split(ccdr.peripheral.GPIOI);

  unsafe {
    cortex_interrupt::enable();
    NVIC::unmask(Interrupt::EXTI0);
  }

  let ticks_per_second = ccdr.clocks.c_ck().to_Hz();

  syst.set_clock_source(SystClkSource::Core);
  syst.set_reload(ticks_per_second / 100); // 10ms
  syst.clear_current();
  syst.enable_counter();

  cortex_interrupt::free(|cs| {
    LEDS
      .borrow(cs)
      .replace(Some(Leds::init(gpiob.pb6, gpiob.pb7, gpioh.ph4, gpioi.pi8)))
  });

  loop {
    for _ in 1..=10 {
      while !syst.has_wrapped() {}
    }

    nvic.request(Interrupt::EXTI0);
  }
}

#[interrupt]
fn EXTI0() {
  cortex_interrupt::free(|cs| {
    let mut leds = LEDS.borrow(cs).borrow_mut();
    let leds = leds.as_mut().unwrap();

    const H: PinState = PinState::High;
    const L: PinState = PinState::Low;
    match leds.get_state() {
      (H, L, L, L) => leds.set_state(L, H, L, L),
      (L, H, L, L) => leds.set_state(L, L, H, L),
      (L, L, H, L) => leds.set_state(L, L, L, H),
      (L, L, L, H) => leds.set_state(H, L, L, L),
      _ => leds.set_state(H, L, L, L),
    }
  });
}
