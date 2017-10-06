#[macro_export]
macro_rules! assert_field_eq (
    ($left: expr, $right: expr, [$($field: ident), *]) => {
        $(
            assert_eq!($left.$field, $right.$field);
        )*
    }
);
