pub mod declarative_macro;
pub mod functions;
mod tests;

use functions::{five, four, one, six, three, two};
use homework_macro::functional_macro;

fn main() {
    // Вызываем макрос с именами функций
    let declarative_result = declarative_macro!(one, two, three, four, five, six);
    println!("{:?}", declarative_result);

    let procedure_result = functional_macro!("one", "two", "three", "four", "five", "six");
    println!("{:?}", procedure_result);
}
