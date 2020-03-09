use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

use crate::field::*;
use datasize::datasize::*;

/// An aggregation of [Field]s, which can be serialized into a
/// [Vec<u32>]
pub struct FieldAggregate {
    fields: Vec<Field>
}

impl FieldAggregate {
    pub fn new() -> FieldAggregate {
        FieldAggregate { fields: Vec::new() }
    }

    /// Collapse a [FieldAggregate] into a u8 Vec.
    pub fn to_buf(&self) -> Vec<u8> {
        let size_bits = self.fields.iter().fold(0, |acc, x| acc + x.size.bits());
        // Add an extra byte if there are bits left over
        let size_bytes = match size_bits {
            s if s % 8 == 0 => size_bits / 8,
            s @ _ => s / 8 + 1
        };
        let mut vec: Vec<u8> = Vec::with_capacity(size_bytes as usize);
        let mut curr_byte = InProgressByte::new();
        for field in &self.fields {
            for bit in field.iter() {
                if !curr_byte.set_next_bit(bit) {
                    vec.push(curr_byte.value);
                    curr_byte = InProgressByte::new();
                    curr_byte.set_next_bit(bit);
                }
            }
        }
        if !curr_byte.empty() {
            vec.push(curr_byte.value);
        }
        vec
    }
}

impl Display for FieldAggregate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let str = self.fields.iter().map(Field::to_string).collect::<Vec<String>>().join(", ");
        write!(f, "{}", str)
    }
}

impl Add<Field> for FieldAggregate {
    type Output = FieldAggregate;

    fn add(mut self, rhs: Field) -> Self::Output {
        self.fields.push(rhs);
        self
    }
}

impl Add for Field {
    type Output = FieldAggregate;

    fn add(self, rhs: Field) -> Self::Output {
        FieldAggregate::new().add(self).add(rhs)
    }
}

#[derive(Debug)]
struct InProgressByte {
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

    /// Add a bit (1 if [bit] is true, 0 otherwise) to this [InProgressByte].
    /// Returns true if there was space in this byte to add the bit, false
    /// if not.
    pub fn set_next_bit(&mut self, is_set: bool) -> bool {
        if self.num_bits == 8 {
            return false;
        }
        if is_set {
            let bit = 1 << (8 - self.num_bits - 1);
            self.value = self.value | bit as u8;
        }
        self.num_bits += 1;
        true
    }

    pub fn empty(&self) -> bool {
        return self.num_bits == 0;
    }
}

impl Display for InProgressByte {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:08b}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let f1 = Field::new(2, bits!(2));
        let f2 = Field::new(3, bits!(2));
        let f3 = Field::new(3, bits!(2));

        let agg = f1 + f2 + f3;
        assert_eq!(agg.fields.len(), 3);
    }

    #[test]
    fn test_to_buf() {
        let vec = (Field::new(2, bits!(2))
            + Field::new(3, bits!(3)))
            .to_buf();
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 152u8);
    }

    #[test]
    fn test_inprogressbyte() {
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

