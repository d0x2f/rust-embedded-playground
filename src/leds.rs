use stm32h7xx_hal::gpio::{Output, Pin, PinState, PushPull};

pub struct Leds {
  led1: Pin<'B', 6, Output<PushPull>>,
  led2: Pin<'B', 7, Output<PushPull>>,
  led3: Pin<'H', 4, Output<PushPull>>,
  led4: Pin<'I', 8, Output<PushPull>>,
}

impl Leds {
  pub fn init(pb6: Pin<'B', 6>, pb7: Pin<'B', 7>, ph4: Pin<'H', 4>, pi8: Pin<'I', 8>) -> Leds {
    Leds {
      led1: pb6.into_push_pull_output(),
      led2: pb7.into_push_pull_output(),
      led3: ph4.into_push_pull_output(),
      led4: pi8.into_push_pull_output(),
    }
  }

  pub fn get_state(&self) -> (PinState, PinState, PinState, PinState) {
    (
      self.led1.get_state(),
      self.led2.get_state(),
      self.led3.get_state(),
      self.led4.get_state(),
    )
  }

  pub fn set_state(
    &mut self,
    led1_state: PinState,
    led2_state: PinState,
    led3_state: PinState,
    led4_state: PinState,
  ) {
    self.led1.set_state(led1_state);
    self.led2.set_state(led2_state);
    self.led3.set_state(led3_state);
    self.led4.set_state(led4_state);
  }
}
