#![allow(dead_code)]
/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при этом цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/

fn digit_product(n: u32) -> u8 {
    
    if n == 10 {
        return 1u8;
    }

    let mut outer_initial = n;

    while outer_initial > 10 {

        let mut digits: Vec<u32> = vec![];
        let mut initial = outer_initial;
        while initial >= 10 {
            let new_digit = initial % 10;
            if new_digit != 0 {
                digits.push(new_digit);
            }
            initial = initial / 10;
        }
        digits.push(initial);
        let result: u32= digits.iter().fold(1, |acc, &x| acc * x);
        outer_initial = result;
    }

    u8::try_from(outer_initial).unwrap()

}


    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(10), 1);
        // assert_eq!(digit_product(100), 1);

        assert_eq!(digit_product(987), 2); // 9*8*7=504, 5*4=20, 2
        assert_eq!(digit_product(123456), 4); // 1*2*3*4*5*6=720, 7*2=14, 1*4=4
        assert_eq!(digit_product(123454321), 6); // 1*2*3*4*5*4*3*2*1=2880, 2*8*8=128, 1*2*8=16, 1*6=6
    }
}

fn main () {
    println!("{}", digit_product(433));
    println!("{}", digit_product(56));


}