#![allow(dead_code)]
/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при это цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/

pub fn digit_product(mut n: u32) -> u8 {
    if n < 10 {
        return n as u8;
    }

    while n >= 10 {

        let mut result = 1;
        while n >= 1 {
            let multiplicator = n % 10;
            if multiplicator != 0 {
                result *= multiplicator;
            }
            n /= 10;
        }

        n = result

    }
    println!("{}", n);
    n as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_product() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2);
        assert_eq!(digit_product(123456), 4);
        assert_eq!(digit_product(123454321), 6);
    }
}
