#[macro_export]
macro_rules! tuple_values {
    ($($func:ident),*) => {
        (
            $($func()),*
        )
    };
}
