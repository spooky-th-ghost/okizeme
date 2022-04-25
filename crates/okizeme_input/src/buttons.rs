#[derive(Debug,Clone, Copy)]
pub struct ButtonPress {
  pub value: u8,
}

impl ButtonPress {
  pub fn new(value: u8) -> Self {
    ButtonPress {
      value
    }
  }

  pub fn any_pressed(&self) -> bool {
    return self.value != 0;
  }

  pub fn to_string(&self) -> String {
    let mut button_string = String::new();
    if self.is_bit_set(0) {button_string.push('A')}
    if self.is_bit_set(1) {button_string.push('B')}
    if self.is_bit_set(2) {button_string.push('C')}
    if self.is_bit_set(3) {button_string.push('D')}
    if self.is_bit_set(4) {button_string.push('E')}
    if self.is_bit_set(5) {button_string.push('F')}
    if self.is_bit_set(6) {button_string.push('G')}
    if self.is_bit_set(7) {button_string.push('H')}
    return button_string;
  }

  pub fn is_button_pressed(&self, button: char) -> bool {
    let shift: u8 = match button {
      'A' => 0,
      'B' => 1,
      'C' => 2,
      'D' => 3,
      'E' => 4,
      'F' => 5,
      'G' => 6,
      'H' => 7,
        _ => return false
    };

    return self.is_bit_set(shift);
  }


  pub fn is_bit_set(&self, position: u8) -> bool {
    return (self.value & (1 << position)) != 0;
  }
}
