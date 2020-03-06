extern crate datasize;
use datasize::*;
use std::fmt::{Display, Debug, Formatter, Result};

mod inprogressbyte;
use inprogressbyte::*;

#[derive(Debug)]
struct Field {
    value: u32,
    size: DataSize
}

/**
  * Return the maximum value that can be held in data_size
  * TODO: support taking by ref and owned? add a macro to handle that?
  */
fn max_value(data_size: &DataSize) -> u32 {
    let mut max_value = 0u32;
    for _ in 0..data_size.bits() - 1 {
        max_value = max_value | 1;
        max_value = max_value << 1;
    }
    // Do the last 'or' here so we don't shift again
    max_value | 1
}

struct FieldIter<'a> {
    curr_index: i32,
    field: &'a Field
}

impl<'a> FieldIter<'a> {
    fn new(field: &Field) -> FieldIter {
        FieldIter { curr_index: (field.size.bits() - 1) as i32, field }
    }
}

/**
  * Iterate over the bits in a Field, from left to right
  */
impl<'a> Iterator for FieldIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_index {
            -1 => None,
            index @ _ => {
                let val = (self.field.value >> index as u32) & 0x1;
                self.curr_index -= 1;
                Some(val)
            }
        }
    }
}

/**
 * A Field describes a value and the DataSize of how large the containing
 * field is for that value.
 */
impl Field {
    fn new(value: u32, size: DataSize) -> Field {
        if value > max_value(&size) {
            panic!("Value {} is too large to fit into {}", value, size);
        }
        Field { value, size }
    }

    fn iter(&self) -> FieldIter {
        FieldIter::new(self)
    }

    fn concat(self, rhs: Self) -> FieldAggregate {
        FieldAggregate::new().concat(self).concat(rhs)
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({})", self.value, self.size)
    }
}

/// An aggregation of [Field]s, which can be serialized into oa
/// [Vec<u32>]
struct FieldAggregate {
    fields: Vec<Field>
}

impl FieldAggregate {
    fn new() -> FieldAggregate {
        FieldAggregate { fields: Vec::new() }
    }

    fn concat(mut self, rhs: Field) -> FieldAggregate {
        self.fields.push(rhs);
        self
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
                let bit_val = match bit {
                    1 => true,
                    0 => false,
                    b @ _ => panic!("invalid bit in field: {}", b)
                };
                if !curr_byte.add_bit(bit_val) {
                    vec.push(curr_byte.value);
                    curr_byte = InProgressByte::new();
                    curr_byte.add_bit(bit_val);
                }
            }
        }
        if !curr_byte.empty() {
            curr_byte.collapse();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_value() {
        assert_eq!(max_value(&bits!(2)), 3);
        assert_eq!(max_value(&bytes!(2)), 65535);
    }

    #[test]
    fn test_create_field() {
        Field::new(3, bits!(2));
    }

    #[test]
    #[should_panic]
    fn test_create_invalid_field() {
        Field::new(10, bits!(2));
    }

    #[test]
    fn test_field_iter() {
        let f = Field::new(2, bits!(2));
        let bits = f.iter().collect::<Vec<u32>>();
        assert_eq!(bits, vec![1, 0]);
    }

    #[test]
    fn test_addition() {
        let f1 = Field::new(2, bits!(2));
        let f2 = Field::new(3, bits!(2));
        let f3 = Field::new(3, bits!(2));

        let agg = f1.concat(f2).concat(f3);
        assert_eq!(agg.fields.len(), 3);
    }

    #[test]
    fn test_to_buf() {
        let vec = Field::new(2, bits!(2))
            .concat(Field::new(3, bits!(3)))
            .to_buf();
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 152u8);
    }
}
