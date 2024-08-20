pub mod functions;
pub mod declarative_macro;

use functions::{one, two, three, four, five, six};
use homework_macro::my_macro;

fn main() {
    // Вызываем макрос с именами функций
    let declarative_result = tuple_values!(one, two, three, four, five, six);
    println!("{:?}", declarative_result);

    let procedure_result = my_macro!("one", "two", "three", "four", "five", "six");
    println!("{:?}", procedure_result);
}
