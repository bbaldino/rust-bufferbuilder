extern crate datasize;
use datasize::*;
use std::fmt::{Display, Debug, Formatter, Result};
use std::ops::Add;

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
        FieldIter { curr_index: field.size.bits() as i32, field }
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
}

impl Add for Field {
    type Output = FieldAggregate;

    fn add(self, rhs: Self) -> Self::Output {
        let mut aggregate = FieldAggregate::new();
        aggregate.add(self);
        aggregate.add(rhs);

        aggregate
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({})", self.value, self.size)
    }
}

/**
 * An aggregation of Fields
 */
struct FieldAggregate {
    fields: Vec<Field>
}

impl FieldAggregate {
    fn new() -> FieldAggregate {
        FieldAggregate { fields: Vec::new() }
    }
}

// TODO: had to implement this for &mut FieldAggregate because, when doing
// it on FieldAggregate and trying to set Output to &FieldAggregate, it wanted
// a lifetime on the Output type and couldn't figure out how to set it
impl Add<Field> for &mut FieldAggregate {
    type Output = Self;

    fn add(self, rhs: Field) -> Self::Output {
        self.fields.push(rhs);
        self
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
        assert_eq!(bits, vec![0, 1, 0]);
    }

    #[test]
    fn test_addition() {
        let f1 = Field::new(2, bits!(2));
        let f2 = Field::new(3, bits!(2));

        println!("{}", f1 + f2);
    }
}
