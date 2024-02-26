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

pub fn fizzbuzz(num: u32) -> String {
    let fizz = String::from("Fizz");
    let buzz = String::from("Buzz");
    let mut result = String::from("");
    if num % 3 == 0 {
        result.push_str(&fizz)
    };
    if num % 5 == 0 {
        result.push_str(&buzz)
    };
    if result.len() == 0 {
        result.push_str(&num.to_string())
    }
    result
}

pub fn fizzbuzz_list(n: u32) -> Vec<String> {
    let mut results = Vec::new();
    for i in 1..n + 1 {
        results.push(fizzbuzz(i));
    }

    println!("{:?}", results);

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(&fizzbuzz(1), "1");
        assert_eq!(&fizzbuzz(3), "Fizz");
        assert_eq!(&fizzbuzz(5), "Buzz");
        assert_eq!(&fizzbuzz(7), "7");
        assert_eq!(&fizzbuzz(9), "Fizz");
        assert_eq!(&fizzbuzz(15), "FizzBuzz");
        assert_eq!(&fizzbuzz(30), "FizzBuzz");
        assert_eq!(&fizzbuzz(49), "49");
    }

    #[test]
    fn test_fizzbuzz_list() {
        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ];
        let result = fizzbuzz_list(15);
        assert_eq!(expected, result);
    }
}
