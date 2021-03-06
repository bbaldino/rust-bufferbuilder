use std::fmt::{Display, Debug, Formatter, Result};

use datasize::datasize::*;
use datasize::fits::*;

/// A [Field] consists of a value and the size (e.g. number of bits) that
/// value should occupy in a buffer.
#[derive(Debug, Copy, Clone)]
pub struct Field {
    pub value: u32,
    pub size: DataSize
}

/// An iterator type for iterating over the bits of a [Field]
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

impl Field {
    /// Create a new [Field] with value [value] and size [size]
    pub fn new(value: u32, size: DataSize) -> Field {
        if !value.fits_in(&size) {
            panic!("Value {} is too large to fit into {}", value, size);
        }
        Field { value, size }
    }

    /// Create an iterator over the bits in this field
    pub fn iter(&self) -> FieldIter {
        FieldIter::new(self)
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({})", self.value, self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate datasize;
    use datasize::*;

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

