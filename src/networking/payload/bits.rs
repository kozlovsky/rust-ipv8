/// This struct represents the bits inside an u8 by unpacking them into booleans.
/// Mostly here to achieve feature parity with py-ipv8
/// see https://github.com/Tribler/py-ipv8/blob/57c1aa73eee8a3b7ee6ad48482fc2e0d5849415e/ipv8/messaging/serialization.py#L84.
#[derive(PartialEq, Debug)]
pub struct Bits {
  pub bit0: bool,
  pub bit1: bool,
  pub bit2: bool,
  pub bit3: bool,
  pub bit4: bool,
  pub bit5: bool,
  pub bit6: bool,
  pub bit7: bool,
}

fn bool_to_u8(data: bool) -> u8 {
  match data {
    true => 1,
    false => 0,
  }
}

impl Bits {
  pub fn new(
    bit0: bool,
    bit1: bool,
    bit2: bool,
    bit3: bool,
    bit4: bool,
    bit5: bool,
    bit6: bool,
    bit7: bool,
  ) -> Self {
    Bits {
      bit0,
      bit1,
      bit2,
      bit3,
      bit4,
      bit5,
      bit6,
      bit7,
    }
  }

  /// convert an u8 into a bits struct.
  pub fn from_u8(num: u8) -> Self {
    Bits {
      bit0: ((num >> 0) & 1) > 0,
      bit1: ((num >> 1) & 1) > 0,
      bit2: ((num >> 2) & 1) > 0,
      bit3: ((num >> 3) & 1) > 0,
      bit4: ((num >> 4) & 1) > 0,
      bit5: ((num >> 5) & 1) > 0,
      bit6: ((num >> 6) & 1) > 0,
      bit7: ((num >> 7) & 1) > 0,
    }
  }

  pub fn to_u8(&self) -> u8 {
    (bool_to_u8(self.bit0) << 0)
      | (bool_to_u8(self.bit1) << 1)
      | (bool_to_u8(self.bit2) << 2)
      | (bool_to_u8(self.bit3) << 3)
      | (bool_to_u8(self.bit4) << 4)
      | (bool_to_u8(self.bit5) << 5)
      | (bool_to_u8(self.bit6) << 6)
      | (bool_to_u8(self.bit7) << 7)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_creation() {
    let b = Bits::new(false, true, false, true, false, true, false, false);
    assert_eq!(b.bit0, false);
    assert_eq!(b.bit1, true);
    assert_eq!(b.bit2, false);
    assert_eq!(b.bit3, true);
    assert_eq!(b.bit4, false);
    assert_eq!(b.bit5, true);
    assert_eq!(b.bit6, false);
    assert_eq!(b.bit7, false);
  }

  #[test]
  fn test_tou8() {
    let b = Bits::new(false, true, false, true, false, true, false, false);
    assert_eq!(b.to_u8(), 42);
  }

  #[test]
  fn test_fromu8() {
    let b = Bits::from_u8(42);
    assert_eq!(b.bit0, false);
    assert_eq!(b.bit1, true);
    assert_eq!(b.bit2, false);
    assert_eq!(b.bit3, true);
    assert_eq!(b.bit4, false);
    assert_eq!(b.bit5, true);
    assert_eq!(b.bit6, false);
    assert_eq!(b.bit7, false);
  }
}
