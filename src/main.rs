#![no_main]
#![no_std]

mod clock_power;
mod leds;
mod timer;

use panic_halt as _;

use cortex_m::interrupt as cortex_interrupt;

use stm32h7xx_hal::gpio::PinState;
use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::{interrupt, pac, pac::Interrupt, pac::NVIC};

use cortex_m_rt::entry;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use clock_power::ClockPower;
use leds::Leds;
use timer::Timer;

static LEDS: Mutex<RefCell<Option<Leds>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
  let p = pac::CorePeripherals::take().unwrap();
  let dp = pac::Peripherals::take().unwrap();

  let mut syst = p.SYST;
  let mut nvic = p.NVIC;

  unsafe {
    NVIC::unmask(Interrupt::EXTI0);
  }

  let clock_power = ClockPower::init(dp.PWR, dp.RCC, dp.SYSCFG);

  let mut timer = Timer::init(
    &mut syst,
    Interrupt::EXTI0,
    u64::from(clock_power.core_speed() / 4),
  );

  let gpiob = dp.GPIOB.split(clock_power.ccdr.peripheral.GPIOB);
  let gpioh = dp.GPIOH.split(clock_power.ccdr.peripheral.GPIOH);
  let gpioi = dp.GPIOI.split(clock_power.ccdr.peripheral.GPIOI);

  cortex_interrupt::free(|cs| {
    LEDS
      .borrow(cs)
      .replace(Some(Leds::init(gpiob.pb6, gpiob.pb7, gpioh.ph4, gpioi.pi8)))
  });

  loop {
    timer.check(&mut nvic)
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
