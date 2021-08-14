// as originally captured by https://github.com/near/core-contracts/blob/master/staking-pool
use uint::construct_uint;
construct_uint! {
  /// 256-bit unsigned integer.
  pub struct U256(4);
}

pub struct SafeFraction {
  pub numerator: u32,
  pub denominator: u32,
}

impl SafeFraction {
  pub fn assert_valid(&self) {
    assert_ne!(self.denominator, 0, "Denominator must be a positive number");
    assert!(
      self.numerator <= self.denominator,
      "The fraction must be less or equal to 1"
    );
  }

  pub fn multiply(&self, value: u128) -> u128 {
    (U256::from(self.numerator) * U256::from(value) / U256::from(self.denominator)).as_u128()
  }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn no_overflow() {
    let fraction = SafeFraction {
      numerator: 9,
      denominator: 10,
    };
    fraction.assert_valid();
    assert_eq!(fraction.multiply(100), 90)
  }
}
