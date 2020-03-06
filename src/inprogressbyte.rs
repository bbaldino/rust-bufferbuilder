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
    pub fn add_bit(&mut self, bit: bool) -> bool {
        if self.num_bits == 8 {
            return false;
        }
        self.num_bits += 1;
        self.value = self.value << 1;
        if bit {
            self.value |= 1;
        }
        true
    }

    /// If an [InProgressByte] isn't complete, but does have data, we'll want
    /// to shift it to the left so that the first set bit is in the MSB
    /// position (since we're building a buffer from left to right)
    /// TODO: Or maybe we should set the bits from left to right to begin with?
    pub fn collapse(&mut self) {
        self.value = self.value << (8 - self.num_bits) as u8;
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
        assert_eq!(x.add_bit(true), true);
        assert_eq!(x.value, 1);

        assert_eq!(x.add_bit(false), true);
        assert_eq!(x.value, 2);

        assert_eq!(x.add_bit(true), true);
        assert_eq!(x.value, 5);

        assert_eq!(x.add_bit(false), true);
        assert_eq!(x.value, 10);

        assert_eq!(x.add_bit(true), true);
        assert_eq!(x.value, 21);

        assert_eq!(x.add_bit(false), true);
        assert_eq!(x.value, 42);

        assert_eq!(x.add_bit(true), true);
        assert_eq!(x.value, 85);

        assert_eq!(x.add_bit(false), true);
        assert_eq!(x.value, 170);

        // Byte is now full
        assert_eq!(x.add_bit(true), false);
        assert_eq!(x.value, 170);
   }

}
