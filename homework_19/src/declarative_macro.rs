#[macro_export]
macro_rules! declarative_macro {
    ($($func:ident),*) => {
        (
            $($func()),*
        )
    };
}
