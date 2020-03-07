use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

use crate::field::*;
use crate::inprogressbyte::*;
use datasize::*;

/// An aggregation of [Field]s, which can be serialized into a
/// [Vec<u32>]
pub struct FieldAggregate {
    fields: Vec<Field>
}

impl FieldAggregate {
    fn new() -> FieldAggregate {
        FieldAggregate { fields: Vec::new() }
    }

    /// Collapse a [FieldAggregate] into a u8 Vec.
    fn to_buf(&self) -> Vec<u8> {
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
}

