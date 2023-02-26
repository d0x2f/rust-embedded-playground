use cortex_m::singleton;
use stm32h7xx_hal::{
  device::{PWR, RCC, SYSCFG},
  prelude::{_stm32h7xx_hal_pwr_PwrExt, _stm32h7xx_hal_rcc_RccExt},
  rcc::Ccdr,
};

pub struct ClockPower {
  pub ccdr: Ccdr,
}

impl ClockPower {
  pub fn init(dp_pwr: PWR, dp_rcc: RCC, dp_syscfg: SYSCFG) -> ClockPower {
    let _x: &'static mut bool = singleton!(TIMER: bool = false).unwrap();

    let pwr = dp_pwr.constrain();
    let pwrcfg = pwr.freeze();
    let rcc = dp_rcc.constrain();
    let ccdr = rcc.freeze(pwrcfg, &dp_syscfg);

    ClockPower { ccdr }
  }

  pub fn core_speed(&self) -> u32 {
    self.ccdr.clocks.c_ck().to_Hz()
  }
}
