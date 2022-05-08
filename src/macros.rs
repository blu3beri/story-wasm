macro_rules! assert_options {
    ($options:expr, $($field:ident)*) => {
        console_error_panic_hook::set_once();
        $(
            if !$options.unwrap().$field.is_some() {
                panic!("{} is required!", stringify!($field));
            }
        )*;
    };
}
