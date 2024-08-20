#[macro_export]
macro_rules! tuple_values {
    ($($func:ident),*) => {
        (
            $($func()),*
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::functions::{five, four, one, six, three, two};

    #[test]
    fn test_tuple_values_full() {
        let x = tuple_values!(one, two, three, four, five, six);
        assert_eq!(x, (1, 2, 3, 4, 5, 6));
    }

    #[test]
    fn test_tuple_values_zero() {
        let x = tuple_values!();
        assert_eq!(x, ());
    }
}
