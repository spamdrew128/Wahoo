#[macro_export]
macro_rules! tuple_constants_enum {
    ($t:ty, $($n:ident),*) => {
        tuple_constants_enum!($t, 0, $($n),*);
    };
    ($t:ty, $val:expr, $name:ident) => {
        pub const $name: $t = <$t>::new($val);
    };
    ($t:ty, $val:expr, $name:ident, $($n:ident),*) => {
        pub const $name: $t = <$t>::new($val);
        tuple_constants_enum!($t, $val + 1, $($n),*);
    };
}
