#![allow(dead_code)]
/*
    Написать функцию, которая превращает число в строку по следующим правилам:
    1. Если число кратно 3, то возвращаем строку "Fizz"
    2. Если число кратно 5, то возвращаем строку "Buzz"
    3. Если число кратно и 3, и 5, то возвращаем строку "FizzBuzz"
    4. В остальных случаях возвращаем строку, содержащую данное число
    
    Написать функцию fizzbuzz_list, которая получает число `n: u32` и возвращает
    список строк, содержащих строковые представления fizzbuzz
    для чисел в диапазоне от 1 до n. Написать тесты.
*/

fn fizzbuzz(num: u32) -> String {
    let mut result = String::from("");
    if num % 3 == 0 {
        result.push_str("Fizz");
    }
    if num % 5 == 0 {
        result.push_str("Buzz");
    }
    if (num % 5 != 0) && (num % 3 != 0) {
        return num.to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(&fizzbuzz(1), "1");
        assert_eq!(&fizzbuzz(3), "Fizz");
        assert_eq!(&fizzbuzz(5), "Buzz");
        assert_eq!(&fizzbuzz(7), "7");
        assert_eq!(&fizzbuzz(9), "Fizz");
        assert_eq!(&fizzbuzz(15), "FizzBuzz");
        assert_eq!(&fizzbuzz(30), "FizzBuzz");
        assert_eq!(&fizzbuzz(49), "49");
    }
}

fn main () {
    print!("{}", &fizzbuzz(5))
}