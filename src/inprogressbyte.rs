use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct InProgressByte {
    num_bits: u32,
    pub value: u8
}

impl InProgressByte {
    pub fn new() -> InProgressByte {
        InProgressByte {
            num_bits: 0,
            value: 0
        }
    }

    /// Add a bit (1 if [bit] is true, 0 otherwise) to the LSB position
    /// of this [InProgressByte].  Returns true if there was space in this
    /// byte to add the bit, false if not.
    pub fn set_next_bit(&mut self, is_set: bool) -> bool {
        if self.num_bits == 8 {
            return false;
        }
        if is_set {
            let bit = 1 << (8 - self.num_bits - 1);
            self.value = self.value | bit as u8;
        }
        // self.value = self.value << 1;
        self.num_bits += 1;
        // if is_set {
        //     self.value |= 1;
        // }
        true
    }

    pub fn empty(&self) -> bool {
        return self.num_bits == 0;
    }
}

impl Display for InProgressByte {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Ideally we'd only pad as many bits as we've set so far, but
        // doing that dynamically isn't supported in stdlib.  Look for
        // a crate?
        write!(f, "{:08b}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_bits() {
        let mut x = InProgressByte::new();
        assert_eq!(x.set_next_bit(true), true);
        assert_eq!(x.value, 0b10000000);

        assert_eq!(x.set_next_bit(false), true);
        assert_eq!(x.value, 0b10000000);

        assert_eq!(x.set_next_bit(true), true);
        assert_eq!(x.value, 0b10100000);

        assert_eq!(x.set_next_bit(false), true);
        assert_eq!(x.value, 0b10100000);

        assert_eq!(x.set_next_bit(true), true);
        assert_eq!(x.value, 0b10101000);

        assert_eq!(x.set_next_bit(false), true);
        assert_eq!(x.value, 0b10101000);

        assert_eq!(x.set_next_bit(true), true);
        assert_eq!(x.value, 0b10101010);

        assert_eq!(x.set_next_bit(false), true);
        assert_eq!(x.value, 0b10101010);

        // Byte is now full
        assert_eq!(x.set_next_bit(true), false);
        assert_eq!(x.value, 0b10101010);
   }
}
