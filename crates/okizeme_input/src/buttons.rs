use std::fmt;

#[derive(Debug,Clone, Copy)]
pub struct ButtonMask(pub u8);

impl Default for ButtonMask {
    fn default() -> Self {
        ButtonMask(0)
    }
}

impl ButtonMask {
  pub fn any(&self) -> bool {
    self.0 != 0
  }

  pub fn contains(&self, button: char) -> bool {
    let shift: u8 = match button {
      'a' => 0,
      'b' => 1,
      'c' => 2,
      'd' => 3,
      'e' => 4,
      'f' => 5,
      'g' => 6,
      'h' => 7,
        _ => return false
    };

    self.is_bit_set(shift)
  }


  fn is_bit_set(&self, position: u8) -> bool {
    (self.0 & (1 << position)) != 0
  }
}

impl fmt::Display for ButtonMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let mut button_string = String::new();
      if self.is_bit_set(0) {button_string.push('a')}
      if self.is_bit_set(1) {button_string.push('b')}
      if self.is_bit_set(2) {button_string.push('c')}
      if self.is_bit_set(3) {button_string.push('d')}
      if self.is_bit_set(4) {button_string.push('e')}
      if self.is_bit_set(5) {button_string.push('f')}
      if self.is_bit_set(6) {button_string.push('g')}
      if self.is_bit_set(7) {button_string.push('h')}
      write!(f,"{}",button_string)
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Buttons {
    pub pressed: ButtonMask,
    pub held: ButtonMask,
    pub released: ButtonMask
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_single_button() {
        let single_button_mask = ButtonMask(0b0000_1000);
        assert!(single_button_mask.contains('d'));
    }

    #[test]
    fn detect_multiple_buttons() {
        let multi_button_mask = ButtonMask(0b1010_0110);
        assert!(
            multi_button_mask.contains('h')
            && multi_button_mask.contains('f')
            && multi_button_mask.contains('b')
            && multi_button_mask.contains('c')
        )
    }


}
