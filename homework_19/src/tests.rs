#[cfg(test)]
mod tests_declarative_macro {
    use crate::declarative_macro;
    use crate::functions::{five, four, one, six, three, two};

    #[test]
    fn test_declarative_macro_full() {
        let x = declarative_macro!(one, two, three, four, five, six);
        assert_eq!(x, (1, 2, 3, 4, 5, 6));
    }

    #[test]
    fn test_declarative_macro_zero() {
        let x = declarative_macro!();
        assert_eq!(x, ());
    }
}

#[cfg(test)]
mod tests_functional_macro {
    use crate::functions::{five, four};
    use homework_macro::functional_macro;

    #[test]
    fn test_declarative_macro_full() {
        let x = functional_macro!("one", "two", "three", "four", "five", "six");
        assert_eq!(x, (4, 5));
    }

    #[test]
    fn test_declarative_macro_zero() {
        let x = functional_macro!();
        assert_eq!(x, ());
    }
}
