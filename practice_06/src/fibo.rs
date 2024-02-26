#![allow(dead_code)]
/*
    Последовательностью Фибоначчи называется последовательность чисел,
    которая удовлетворяет следующим условиям:
    - элемент последовательности с индексом 0 - число 0
    - элемент с индексом 1 - число 1
    - каждый последующий элемент равен сумме двух предыдущих.

    0, 1, 1, 2, 3, 5, 8, 13, 21 ...

    Написать функцию, которая вычислит элемент последовательности с индексом n.

    * Написать вторую функцию, которая вернёт последовательность Фибонначи
      от первого элемента до n-ого. Написать тесты
*/

pub fn fib(n: u32) -> u32 {
    let mut current = 0u32;
    let mut next = 1u32;

    for _ in 0..n {
        let old = current;
        current = next;
        next = old + current;
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(7), 13);
    }
}
