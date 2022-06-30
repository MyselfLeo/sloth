pub mod io;

#[macro_export]
macro_rules! get_builtin {
    ($submod:literal, $name:literal) => {
        builtin:: :: paste::paste! {[Builtin $submod:camel $name:camel>] {}}
    };
}