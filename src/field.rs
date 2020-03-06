extern crate datasize;
use datasize::*;

use std::fmt::{Display, Debug, Formatter, Result};

#[derive(Debug)]
pub struct Field {
    pub value: u32,
    pub size: DataSize
}

pub struct FieldIter<'a> {
    curr_index: i32,
    field: &'a Field
}

impl<'a> FieldIter<'a> {
    fn new(field: &Field) -> FieldIter {
        FieldIter { curr_index: (field.size.bits() - 1) as i32, field }
    }
}

/// A wrapper for a u32 such that we can define [Into<bool>] to easily
/// convert a masked bit into a bool.
/// TODO: could this be useful to use elsewhere?
struct Bit(u32);

impl Into<bool> for Bit {
    fn into(self) -> bool {
        match self.0 {
            0 => false,
            _ => true
        }
    }
}

/// Iterate over the bits in a [Field] as [bool]s ([true] if the bit
/// is set, [false] if not)
impl<'a> Iterator for FieldIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_index {
            -1 => None,
            index @ _ => {
                let val = (self.field.value >> index as u32) & 0x1;
                self.curr_index -= 1;
                Some(Bit(val).into())
            }
        }
    }
}

/**
 * A Field describes a value and the DataSize of how large the containing
 * field is for that value.
 */
impl Field {
    pub fn new(value: u32, size: DataSize) -> Field {
        if value > max_value(&size) {
            panic!("Value {} is too large to fit into {}", value, size);
        }
        Field { value, size }
    }

    pub fn iter(&self) -> FieldIter {
        FieldIter::new(self)
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({})", self.value, self.size)
    }
}

/**
  * Return the maximum value that can be held in data_size
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
        let bits = f.iter().collect::<Vec<bool>>();
        assert_eq!(bits, vec![true, false]);
    }
}

