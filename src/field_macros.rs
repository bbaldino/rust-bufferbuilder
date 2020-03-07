use crate::field::*;
use datasize::*;

/// TODO: is there a way we can avoid defining this for every type (which
/// datasize! already does)?
#[macro_export]
macro_rules! field {
    ($value:expr, $size:literal bits) => {
        Field::new($value, datasize!($size bits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = field!(2, 2 bits);
        assert_eq!(x.value, 2);
        assert_eq!(x.size, datasize!(2 bits));
    }
}
