pub type SignedCounter = isize;
pub type UnsignedCounter = usize;

pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_signed_counter() {
        let result = default_signed_counter();
        let expected: SignedCounter = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_default_unsigned_counter() {
        let result = default_unsigned_counter();
        let expected: UnsignedCounter = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_signed() {
        let result = next_signed(default_signed_counter());
        let expected: SignedCounter = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_prev_signed() {
        let result = prev_signed(default_signed_counter());
        let expected: SignedCounter = -1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_unsigned() {
        let result = next_unsigned(default_unsigned_counter());
        let expected: UnsignedCounter = 1;
        assert_eq!(result, expected);
    }
}
