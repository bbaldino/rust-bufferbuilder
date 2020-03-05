extern crate datasize;
use datasize::*;
use std::fmt::{Display, Formatter, Result};

struct Field {
    size: DataSize,
    value: u32
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

impl FieldIter<'_> {
    fn new(field: &Field) -> FieldIter {
        FieldIter { curr_index: field.size.bits() as i32, field }
    }
}

/**
  * Iterate over the bits in a Field, from left to right
  * TODO: read more about 'anonymous lifetime' ('_)
  */
impl Iterator for FieldIter<'_> {
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
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({})", self.value, self.size)
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
        assert_eq!(bits, vec![0, 1, 0]);
    }
}
