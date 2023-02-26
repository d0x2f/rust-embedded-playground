use cortex_m::{
  peripheral::{syst::SystClkSource, NVIC},
  singleton,
};
use stm32h7xx_hal::pac::{Interrupt, SYST};

enum TimerState {
  WaitingForFullCycle,
  WaitingForRemainder,
}

pub struct Timer<'a> {
  syst: &'a mut SYST,
  interrupt: Interrupt,
  full_cycles: u64,
  full_cycles_remaining: u64,
  remainder: u32,
  state: TimerState,
}

impl Timer<'_> {
  // Trigger the given interrupt every interval.
  // Uses the SysTimer peripheral.
  pub fn init(syst: &mut SYST, interrupt: Interrupt, interval: u64) -> Timer {
    let _x: &'static mut bool = singleton!(TIMER: bool = false).unwrap();

    let full_cycles = interval >> 24;
    let remainder = 0x00000000_00FFFFFF & interval;

    syst.set_clock_source(SystClkSource::Core);

    let mut timer = Timer {
      syst,
      interrupt,
      full_cycles,
      full_cycles_remaining: full_cycles,
      remainder: remainder.try_into().unwrap(),
      state: TimerState::WaitingForFullCycle,
    };
    timer.reset_full_cycle();
    timer
  }

  fn reset_full_cycle(&mut self) {
    self.full_cycles_remaining = self.full_cycles;
    self.syst.set_reload(0x00FFFFFE);
    self.syst.clear_current();
    self.syst.enable_counter();
    self.state = TimerState::WaitingForFullCycle
  }

  fn reset_remainder(&mut self) {
    self.syst.set_reload(self.remainder - 1);
    self.syst.clear_current();
    self.syst.enable_counter();
    self.state = TimerState::WaitingForRemainder
  }

  fn check_full_cycle(&mut self) {
    if self.syst.has_wrapped() && self.full_cycles_remaining != 0 {
      self.full_cycles_remaining -= 1;
    }
  }

  fn check_remainder(&mut self, nvic: &mut NVIC) {
    if self.syst.has_wrapped() {
      self.reset_full_cycle();
      nvic.request(self.interrupt);
    }
  }

  pub fn check(&mut self, nvic: &mut NVIC) {
    match self.state {
      TimerState::WaitingForFullCycle => {
        self.check_full_cycle();
        if self.full_cycles_remaining == 0 {
          self.reset_remainder();
        }
      }
      TimerState::WaitingForRemainder => self.check_remainder(nvic),
    }
  }
}
