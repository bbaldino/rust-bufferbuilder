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

impl Field {
    fn new(value: u32, size: DataSize) -> Field {
        if value > max_value(&size) {
            panic!("Value {} is too large to fit into {}", value, size);
        }
        Field { value, size }
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
}
