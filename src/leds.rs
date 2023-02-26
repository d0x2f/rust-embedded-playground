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

  pub fn get_states(&self) -> (PinState, PinState, PinState, PinState) {
    (
      self.led1.get_state(),
      self.led2.get_state(),
      self.led3.get_state(),
      self.led4.get_state(),
    )
  }

  pub fn get_states_as_u8(&self) -> u8 {
    let mut i = 0;

    if self.led1.is_set_high() {
      i += 1;
    }

    if self.led2.is_set_high() {
      i += 2;
    }

    if self.led3.is_set_high() {
      i += 4;
    }

    if self.led4.is_set_high() {
      i += 8;
    }

    i
  }

  pub fn set_states_as_u8(&mut self, i: u8) {
    Self::set_led_state_bool(&mut self.led1, i & 1 == 1);
    Self::set_led_state_bool(&mut self.led2, i & 1 << 1 == 1 << 1);
    Self::set_led_state_bool(&mut self.led3, i & 1 << 2 == 1 << 2);
    Self::set_led_state_bool(&mut self.led4, i & 1 << 3 == 1 << 3);
  }

  fn set_led_state_bool<const P: char, const N: u8>(
    led: &mut Pin<P, N, Output<PushPull>>,
    high: bool,
  ) {
    if high {
      led.set_high()
    } else {
      led.set_low()
    }
  }

  pub fn set_states(
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
